#!/usr/bin/env ruby

require 'erb'

MAX_LOCAL_VARIABLES = 100

def initialize_stack(stack_name)
  <<~LIQUID
  {%- assign #{stack_name} = "__BASE__" -%}
  {%- assign #{stack_name}_top = 0 -%}
  {%- assign #{stack_name}_last_len = 0 -%}
  LIQUID
end

def push(stack_name, input)
  <<~LIQUID
  {%- assign #{stack_name}_top = #{stack_name}_top | plus: 1 -%}
  {%- assign #{stack_name} = #{stack_name} | append: " " | append: #{input} -%}
  LIQUID
end

def pop(stack_name, output)
  <<~LIQUID
  {%- assign #{stack_name}_array = #{stack_name} | split: " " -%}
  {%- assign #{output} = #{stack_name}_array | last %}
  {%- assign #{stack_name} = #{stack_name} | truncatewords: #{stack_name}_top | remove: "..." -%}
  {%- assign #{stack_name}_top = #{stack_name}_top | minus: 1 %}
  LIQUID
end

def cast_int(value)
  "{%- assign #{value} = #{value} | plus: 0 -%}"
end

def build_stack_frame(output_name)
  stack_frame_expression = 'var0'
  (1...MAX_LOCAL_VARIABLES).each do |i|
    stack_frame_expression << " | append: '/' | append: var#{i}"
  end
  "{%- assign #{output_name} = #{stack_frame_expression} %}"
end

def load_stack_frame(input_name)
  assignments = ""
  MAX_LOCAL_VARIABLES.times do |i|
    assignments << "{% assign var#{i} = __split[#{i}] %}"
  end
  "{% assign __split = #{input_name} | split: '/' %}#{assignments}"
end


template = ERB.new(DATA.read)
puts template.result(binding)

__END__

{%- assign constants = "__CONSTANTS__" | split: " "  -%}
{%- assign bytecode = "__BYTECODE__" -%}
{%- assign ops = bytecode | split: " " -%}

{% capture newline %}
{% endcapture %}


<%#
  The stack separate elements based on space, since this makes it possible to POP
  using truncatewords.

  However, truncatewords when called with 0 returns a single word, so we need to
  store at least 1 element at all times, and ignore it. This is the "__BASE__"
  element. Unfortunately, this means all indexes are 1 based for the stack :<
%>

<%= initialize_stack('data_stack') %>
<%= initialize_stack('var_stack') %>
<%= initialize_stack('return_stack') %>

{%- assign pc = 0 -%}
{%- assign cycle_count = 0 -%}
{%- assign output = "" -%}

<%# Local variables, yay! %>
<% MAX_LOCAL_VARIABLES.times do |i| %>
  {%- assign var<%= i%> = "" -%}
<% end %>

{%- for _tick in (1..12345678) -%}
    {%- assign cycle_count = cycle_count | plus: 1 -%}

  	{%- assign op_str = ops[pc] -%}
    {%- assign op = op_str | split: "/" -%}
    {%- assign op_type = op[0] -%}
  
    {%- assign next_pc = pc | plus: 1 %}
  
    {%- case op_type -%}

    <%# push constant %>
  	{%- when "c" -%}
      {%- assign constant_index = op[1] | plus: 0 -%}
      {%- assign constant_value = constants[constant_index] -%}
      <%= push('data_stack', 'constant_value') %>

    <%# add %>
    {%- when "+" -%}
      <%= pop('data_stack', 'second') %>
      <%= pop('data_stack', 'first') %>
      {%- assign result = first | plus: second %}
      <%= push('data_stack', 'result') %>

    <%# subtract %>
    {%- when "-" -%}
      <%= pop('data_stack', 'second') %>
      <%= pop('data_stack', 'first') %>
      {%- assign result = first | minus: second %}
      <%= push('data_stack', 'result') %>

    <%# modulo %>
    {%- when "%" -%}
      <%= pop('data_stack', 'second') %>
      <%= pop('data_stack', 'first') %>
      {%- assign result = first | modulo: second %}
      <%= push('data_stack', 'result') %>

    <%# greater than %>
    {%- when ">" -%}
      <%= pop('data_stack', 'first') %>
      <%= pop('data_stack', 'second') %>
      <%= cast_int('first') %>
      <%= cast_int('second') %>
      {%- if second > first  -%}
        <%= push('data_stack', '1') %>
      {%- else -%}
        <%= push('data_stack', '0') %>
      {%- endif -%}

    <%# less than %>
    {%- when "<" -%}
      <%= pop('data_stack', 'first') %>
      <%= pop('data_stack', 'second') %>
      <%= cast_int('first') %>
      <%= cast_int('second') %>
      {%- if second < first  -%}
        <%= push('data_stack', '1') %>
      {%- else -%}
        <%= push('data_stack', '0') %>
      {%- endif -%}

    <%# or %>
    {%- when "|" -%}
      <%= pop('data_stack', 'first') %>
      <%= pop('data_stack', 'second') %>
      <%= cast_int('first') %>
      <%= cast_int('second') %>
      {%- if first == 1 or second == 1 -%}
        PUSH(data_stack, 1)
      {%- else -%}
        PUSH(data_stack, 0)
      {%- endif -%}

    <%# or %>
    {%- when "&" -%}
      <%= pop('data_stack', 'first') %>
      <%= pop('data_stack', 'second') %>
      <%= cast_int('first') %>
      <%= cast_int('second') %>
      {%- if first == 1 and second == 1 -%}
        PUSH(data_stack, 1)
      {%- else -%}
        PUSH(data_stack, 0)
      {%- endif -%}

    <%# not %>
    {%- when "!" -%}
      <%= pop('data_stack', 'top') %>
      <%= cast_int('top') %>
      {%- if top == 1 -%}
        <%= push('data_stack', 0) %>
      {%- else -%}
        <%= push('data_stack', 1) %>
      {%- endif -%}

    <%# multiply %>
    {%- when "*" -%}
      <%= pop('data_stack', 'first') %>
      <%= pop('data_stack', 'second') %>
      {%- assign result = first | times: second %}
      <%= push('data_stack', 'result') %>

    <%# divide %>
    {%- when "÷" -%}
      <%= pop('data_stack', 'second') %>
      <%= pop('data_stack', 'first') %>
      {%- assign result = first | divided_by: second %}
      <%= push('data_stack', 'result') %>

    <%# echo %>
    {%- when "e" -%}
      <%= pop('data_stack', 'popped') %>
      {%- assign output = output | append: popped | append: newline -%}

    <%# equality check %>
    {%- when "=" -%}
      <%= pop('data_stack', 'first') %>
      <%= pop('data_stack', 'second') %>
  
      {%- assign result = 0 -%}
      {%- if first == second -%}
        {%- assign result = 1 %}
      {%- endif -%}

      <%= push('data_stack', 'result') %>

    <%# jump if true %>
  	{%- when "j" -%}
      {%- assign target = op[1] -%}
      <%= cast_int('target') %>

      <%= pop('data_stack', 'conditon') %>
      <%= cast_int('condition') %>

      {%- if condition == 1 -%}
        {%- assign next_pc = target -%}
      {%- endif -%}

    <%# goto %>
  	{%- when "g" -%}
      {%- assign target = op[1] -%}
      <%= cast_int('target') %>
      {%- assign next_pc = target -%}

    <%# call subroutine %>
    {%- when "k" -%}
      {%- assign target = op[1] -%}
      <%= cast_int('target') %>
      <%= build_stack_frame('stack_frame') %>
      <%= push('var_stack', 'stack_frame') %>
      {%- assign continue_pc = pc | plus: 1 -%}
      <%= push('return_stack', 'continue_pc') %>
      {%- assign next_pc = target -%}

    <%# set a variable %>
    {%- when "s" -%}
      {%- assign var_num = op[1] -%}
      <%= cast_int('var_num') %>
      <%= pop('data_stack', 'new_value') %>

      {%- case var_num -%}
        <% MAX_LOCAL_VARIABLES.times do |i| %>
          {%- when <%= i %> %}
            {%- assign var<%= i %> = new_value -%}
        <% end %>
      {%- endcase -%}

    <%# push a variable %>
    {%- when "v" -%}
      {%- assign var_num = op[1] -%}
      <%= cast_int('var_num') %>
      {%- assign loaded_value = -1337 -%}
      {%- case var_num -%}
        <% MAX_LOCAL_VARIABLES.times do |i| %>
          {%- when <%= i %> -%}
            {%- assign loaded_value = var<%= i %> -%}
        <% end %>
      {%- endcase -%}
      <%= push('data_stack', 'loaded_value') %>

    <%# return %>
  	{%- when "r" -%}
      {%- if return_stack_top == 0 -%} 
        {%- break -%}
      {%- else -%}
        <%= pop('var_stack', 'stack_frame') %>
        <%= load_stack_frame('stack_frame') %>
        <%= pop('return_stack', 'return_pc') %>
        {%- assign next_pc = return_pc -%}
      {%- endif -%}
    {% endcase -%}

  	{%- assign pc = next_pc -%}
{%- endfor -%}
<code>
constants: {{ constants | join: ", " }}
bytecode: {{ bytecode }}
--- Output ----------------------------------------------------------
{{ output }}
---------------------------------------------------------------------
Finished in {{ cycle_count }} cycles
End stack: {{ data_stack }}
Stack top: {{ data_stack_top }}
PC: {{ pc }}
</code>

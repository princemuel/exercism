get_item(stack, position) = stack[position]

function set_item!(stack, position, item)
    stack[position] = item
    stack
end

insert_item_at_top! = push!

remove_item! = deleteat!

function remove_item_from_top!(stack)
    pop!(stack)
    stack
end

function insert_item_at_bottom!(stack, item)
    pushfirst!(stack, item)
    stack
end

function remove_item_at_bottom!(stack)
    popfirst!(stack)
    stack
end

check_size_of_stack(stack, stack_size) = length(stack) == stack_size


can_do_fast_attack(knight_awake::Bool) = !knight_awake

can_spy(knight_awake::Bool, archer_awake::Bool, prisoner_awake::Bool) = knight_awake || archer_awake || prisoner_awake

can_signal_prisoner(archer_awake::Bool, prisoner_awake::Bool) = prisoner_awake && !archer_awake

can_free_prisoner(knight_awake::Bool, archer_awake::Bool, prisoner_awake::Bool, dog_present::Bool) = (dog_present && !archer_awake) || (prisoner_awake && !knight_awake && !archer_awake)


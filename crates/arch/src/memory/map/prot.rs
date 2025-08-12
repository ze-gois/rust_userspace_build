::macros::enum_flag!(Prot,usize,"Protection flags",[
    [0; None; NONE; "None"; "None"],
    [1; Read; READ; "Read"; "Read"],
    [2; Write; WRITE; "Write"; "Write"],
    [4; Exec; EXEC; "Exec"; "Exec"]
]);

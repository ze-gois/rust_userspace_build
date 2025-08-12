#[rustfmt::skip]
::macros::enum_flag!(Flag, usize, "Map Flags",[
    [0x01;      Shared;         SHARED;          "Shared";         "Shared"],
    [0x02;      Private;        PRIVATE;         "Private";        "Private"],
    [0x03;      SharedValidate; SHAREDVALIDATE;  "SharedValidate"; "SharedValidate"],
    [0x10;      Fixed;          FIXED;           "Fixed";          "Fixed"],
    [0x20;      Anonymous;      ANONYMOUS;       "Anonymous";      "Anonymous"],
    [0x0100;    GrowsDown;      GROWSDOWN;       "GrowsDown";      "GrowsDown"],
    [0x0800;    DenyWrite;      DENYWRITE;       "DenyWrite";      "DenyWrite"],
    [0x1000;    Executable;     EXECUTABLE;      "Executable";     "Executable"],
    [0x2000;    Locked;         LOCKED;          "Locked";         "Locked"],
    [0x4000;    NoReserve;      NORESERVE;       "NoReserve";      "NoReserve"],
    [0x8000;    Populate;       POPULATE;        "Populate";       "Populate"],
    [0x10000;   NonBlock;       NONBLOCK;        "NonBlock";       "NonBlock"],
    [0x20000;   Stack;          STACK;           "Stack";          "Stack"],
    [0x40000;   HugeTlb;        HUGETLB;         "HugeTlb";        "HugeTlb"],
    [0x80000;   Sync;           SYNC;            "Sync";           "Sync"],
    [0x100000;  FixedNoReplace; FIXEDNOREPLACE;  "FixedNoReplace"; "FixedNoReplace"]
]);

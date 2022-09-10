// Copyright (c) 2022 John Millikin <john@john-millikin.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
// SPDX-License-Identifier: 0BSD

#![allow(non_upper_case_globals)]

syscall_constants! {
	SYS_exit                         = 1,
	SYS_fork                         = 2,
	SYS_read                         = 3,
	SYS_write                        = 4,
	SYS_open                         = 5,
	SYS_close                        = 6,
	SYS_waitpid                      = 7,
	SYS_creat                        = 8,
	SYS_link                         = 9,
	SYS_unlink                       = 10,
	SYS_execve                       = 11,
	SYS_chdir                        = 12,
	SYS_time                         = 13,
	SYS_mknod                        = 14,
	SYS_chmod                        = 15,
	SYS_lchown                       = 16,
	SYS_oldstat                      = 18,
	SYS_lseek                        = 19,
	SYS_getpid                       = 20,
	SYS_mount                        = 21,
	SYS_umount                       = 22,
	SYS_setuid                       = 23,
	SYS_getuid                       = 24,
	SYS_stime                        = 25,
	SYS_ptrace                       = 26,
	SYS_alarm                        = 27,
	SYS_oldfstat                     = 28,
	SYS_pause                        = 29,
	SYS_utime                        = 30,
	SYS_access                       = 33,
	SYS_nice                         = 34,
	SYS_sync                         = 36,
	SYS_kill                         = 37,
	SYS_rename                       = 38,
	SYS_mkdir                        = 39,
	SYS_rmdir                        = 40,
	SYS_dup                          = 41,
	SYS_pipe                         = 42,
	SYS_times                        = 43,
	SYS_brk                          = 45,
	SYS_setgid                       = 46,
	SYS_getgid                       = 47,
	SYS_signal                       = 48,
	SYS_geteuid                      = 49,
	SYS_getegid                      = 50,
	SYS_acct                         = 51,
	SYS_umount2                      = 52,
	SYS_ioctl                        = 54,
	SYS_fcntl                        = 55,
	SYS_setpgid                      = 57,
	SYS_oldolduname                  = 59,
	SYS_umask                        = 60,
	SYS_chroot                       = 61,
	SYS_ustat                        = 62,
	SYS_dup2                         = 63,
	SYS_getppid                      = 64,
	SYS_getpgrp                      = 65,
	SYS_setsid                       = 66,
	SYS_sigaction                    = 67,
	SYS_sgetmask                     = 68,
	SYS_ssetmask                     = 69,
	SYS_setreuid                     = 70,
	SYS_setregid                     = 71,
	SYS_sigsuspend                   = 72,
	SYS_sigpending                   = 73,
	SYS_sethostname                  = 74,
	SYS_setrlimit                    = 75,
	SYS_getrlimit                    = 76,
	SYS_getrusage                    = 77,
	SYS_gettimeofday                 = 78,
	SYS_settimeofday                 = 79,
	SYS_getgroups                    = 80,
	SYS_setgroups                    = 81,
	SYS_select                       = 82,
	SYS_symlink                      = 83,
	SYS_oldlstat                     = 84,
	SYS_readlink                     = 85,
	SYS_swapon                       = 87,
	SYS_reboot                       = 88,
	SYS_readdir                      = 89,
	SYS_mmap                         = 90,
	SYS_munmap                       = 91,
	SYS_truncate                     = 92,
	SYS_ftruncate                    = 93,
	SYS_fchmod                       = 94,
	SYS_fchown                       = 95,
	SYS_getpriority                  = 96,
	SYS_setpriority                  = 97,
	SYS_statfs                       = 99,
	SYS_fstatfs                      = 100,
	SYS_ioperm                       = 101,
	SYS_socketcall                   = 102,
	SYS_syslog                       = 103,
	SYS_setitimer                    = 104,
	SYS_getitimer                    = 105,
	SYS_stat                         = 106,
	SYS_lstat                        = 107,
	SYS_fstat                        = 108,
	SYS_olduname                     = 109,
	SYS_iopl                         = 110,
	SYS_vhangup                      = 111,
	SYS_vm86old                      = 113,
	SYS_wait4                        = 114,
	SYS_swapoff                      = 115,
	SYS_sysinfo                      = 116,
	SYS_ipc                          = 117,
	SYS_fsync                        = 118,
	SYS_sigreturn                    = 119,
	SYS_clone                        = 120,
	SYS_setdomainname                = 121,
	SYS_uname                        = 122,
	SYS_modify_ldt                   = 123,
	SYS_adjtimex                     = 124,
	SYS_mprotect                     = 125,
	SYS_sigprocmask                  = 126,
	SYS_init_module                  = 128,
	SYS_delete_module                = 129,
	SYS_quotactl                     = 131,
	SYS_getpgid                      = 132,
	SYS_fchdir                       = 133,
	SYS_personality                  = 136,
	SYS_setfsuid                     = 138,
	SYS_setfsgid                     = 139,
	SYS__llseek                      = 140,
	SYS_getdents                     = 141,
	SYS__newselect                   = 142,
	SYS_flock                        = 143,
	SYS_msync                        = 144,
	SYS_readv                        = 145,
	SYS_writev                       = 146,
	SYS_getsid                       = 147,
	SYS_fdatasync                    = 148,
	SYS_mlock                        = 150,
	SYS_munlock                      = 151,
	SYS_mlockall                     = 152,
	SYS_munlockall                   = 153,
	SYS_sched_setparam               = 154,
	SYS_sched_getparam               = 155,
	SYS_sched_setscheduler           = 156,
	SYS_sched_getscheduler           = 157,
	SYS_sched_yield                  = 158,
	SYS_sched_get_priority_max       = 159,
	SYS_sched_get_priority_min       = 160,
	SYS_sched_rr_get_interval        = 161,
	SYS_nanosleep                    = 162,
	SYS_mremap                       = 163,
	SYS_setresuid                    = 164,
	SYS_getresuid                    = 165,
	SYS_vm86                         = 166,
	SYS_poll                         = 168,
	SYS_setresgid                    = 170,
	SYS_getresgid                    = 171,
	SYS_prctl                        = 172,
	SYS_rt_sigreturn                 = 173,
	SYS_rt_sigaction                 = 174,
	SYS_rt_sigprocmask               = 175,
	SYS_rt_sigpending                = 176,
	SYS_rt_sigtimedwait              = 177,
	SYS_rt_sigqueueinfo              = 178,
	SYS_rt_sigsuspend                = 179,
	SYS_pread64                      = 180,
	SYS_pwrite64                     = 181,
	SYS_chown                        = 182,
	SYS_getcwd                       = 183,
	SYS_capget                       = 184,
	SYS_capset                       = 185,
	SYS_sigaltstack                  = 186,
	SYS_sendfile                     = 187,
	SYS_vfork                        = 190,
	SYS_ugetrlimit                   = 191,
	SYS_mmap2                        = 192,
	SYS_truncate64                   = 193,
	SYS_ftruncate64                  = 194,
	SYS_stat64                       = 195,
	SYS_lstat64                      = 196,
	SYS_fstat64                      = 197,
	SYS_lchown32                     = 198,
	SYS_getuid32                     = 199,
	SYS_getgid32                     = 200,
	SYS_geteuid32                    = 201,
	SYS_getegid32                    = 202,
	SYS_setreuid32                   = 203,
	SYS_setregid32                   = 204,
	SYS_getgroups32                  = 205,
	SYS_setgroups32                  = 206,
	SYS_fchown32                     = 207,
	SYS_setresuid32                  = 208,
	SYS_getresuid32                  = 209,
	SYS_setresgid32                  = 210,
	SYS_getresgid32                  = 211,
	SYS_chown32                      = 212,
	SYS_setuid32                     = 213,
	SYS_setgid32                     = 214,
	SYS_setfsuid32                   = 215,
	SYS_setfsgid32                   = 216,
	SYS_pivot_root                   = 217,
	SYS_mincore                      = 218,
	SYS_madvise                      = 219,
	SYS_getdents64                   = 220,
	SYS_fcntl64                      = 221,
	SYS_gettid                       = 224,
	SYS_readahead                    = 225,
	SYS_setxattr                     = 226,
	SYS_lsetxattr                    = 227,
	SYS_fsetxattr                    = 228,
	SYS_getxattr                     = 229,
	SYS_lgetxattr                    = 230,
	SYS_fgetxattr                    = 231,
	SYS_listxattr                    = 232,
	SYS_llistxattr                   = 233,
	SYS_flistxattr                   = 234,
	SYS_removexattr                  = 235,
	SYS_lremovexattr                 = 236,
	SYS_fremovexattr                 = 237,
	SYS_tkill                        = 238,
	SYS_sendfile64                   = 239,
	SYS_futex                        = 240,
	SYS_sched_setaffinity            = 241,
	SYS_sched_getaffinity            = 242,
	SYS_set_thread_area              = 243,
	SYS_get_thread_area              = 244,
	SYS_io_setup                     = 245,
	SYS_io_destroy                   = 246,
	SYS_io_getevents                 = 247,
	SYS_io_submit                    = 248,
	SYS_io_cancel                    = 249,
	SYS_fadvise64                    = 250,
	SYS_exit_group                   = 252,
	SYS_lookup_dcookie               = 253,
	SYS_epoll_create                 = 254,
	SYS_epoll_ctl                    = 255,
	SYS_epoll_wait                   = 256,
	SYS_set_tid_address              = 258,
	SYS_timer_create                 = 259,
	SYS_timer_settime                = 260,
	SYS_timer_gettime                = 261,
	SYS_timer_getoverrun             = 262,
	SYS_timer_delete                 = 263,
	SYS_clock_settime                = 264,
	SYS_clock_gettime                = 265,
	SYS_clock_getres                 = 266,
	SYS_clock_nanosleep              = 267,
	SYS_statfs64                     = 268,
	SYS_fstatfs64                    = 269,
	SYS_tgkill                       = 270,
	SYS_utimes                       = 271,
	SYS_fadvise64_64                 = 272,
	SYS_mbind                        = 274,
	SYS_get_mempolicy                = 275,
	SYS_set_mempolicy                = 276,
	SYS_mq_open                      = 277,
	SYS_mq_unlink                    = 278,
	SYS_mq_timedsend                 = 279,
	SYS_mq_timedreceive              = 280,
	SYS_mq_notify                    = 281,
	SYS_mq_getsetattr                = 282,
	SYS_kexec_load                   = 283,
	SYS_waitid                       = 284,
	SYS_add_key                      = 286,
	SYS_request_key                  = 287,
	SYS_keyctl                       = 288,
	SYS_ioprio_set                   = 289,
	SYS_ioprio_get                   = 290,
	SYS_inotify_init                 = 291,
	SYS_inotify_add_watch            = 292,
	SYS_inotify_rm_watch             = 293,
	SYS_migrate_pages                = 294,
	SYS_openat                       = 295,
	SYS_mkdirat                      = 296,
	SYS_mknodat                      = 297,
	SYS_fchownat                     = 298,
	SYS_futimesat                    = 299,
	SYS_fstatat64                    = 300,
	SYS_unlinkat                     = 301,
	SYS_renameat                     = 302,
	SYS_linkat                       = 303,
	SYS_symlinkat                    = 304,
	SYS_readlinkat                   = 305,
	SYS_fchmodat                     = 306,
	SYS_faccessat                    = 307,
	SYS_pselect6                     = 308,
	SYS_ppoll                        = 309,
	SYS_unshare                      = 310,
	SYS_set_robust_list              = 311,
	SYS_get_robust_list              = 312,
	SYS_splice                       = 313,
	SYS_sync_file_range              = 314,
	SYS_tee                          = 315,
	SYS_vmsplice                     = 316,
	SYS_move_pages                   = 317,
	SYS_getcpu                       = 318,
	SYS_epoll_pwait                  = 319,
	SYS_utimensat                    = 320,
	SYS_signalfd                     = 321,
	SYS_timerfd_create               = 322,
	SYS_eventfd                      = 323,
	SYS_fallocate                    = 324,
	SYS_timerfd_settime              = 325,
	SYS_timerfd_gettime              = 326,
	SYS_signalfd4                    = 327,
	SYS_eventfd2                     = 328,
	SYS_epoll_create1                = 329,
	SYS_dup3                         = 330,
	SYS_pipe2                        = 331,
	SYS_inotify_init1                = 332,
	SYS_preadv                       = 333,
	SYS_pwritev                      = 334,
	SYS_rt_tgsigqueueinfo            = 335,
	SYS_perf_event_open              = 336,
	SYS_recvmmsg                     = 337,
	SYS_fanotify_init                = 338,
	SYS_fanotify_mark                = 339,
	SYS_prlimit64                    = 340,
	SYS_name_to_handle_at            = 341,
	SYS_open_by_handle_at            = 342,
	SYS_clock_adjtime                = 343,
	SYS_syncfs                       = 344,
	SYS_sendmmsg                     = 345,
	SYS_setns                        = 346,
	SYS_process_vm_readv             = 347,
	SYS_process_vm_writev            = 348,
	SYS_kcmp                         = 349,
	SYS_finit_module                 = 350,
	SYS_sched_setattr                = 351,
	SYS_sched_getattr                = 352,
	SYS_renameat2                    = 353,
	SYS_seccomp                      = 354,
	SYS_getrandom                    = 355,
	SYS_memfd_create                 = 356,
	SYS_bpf                          = 357,
	SYS_execveat                     = 358,
	SYS_socket                       = 359,
	SYS_socketpair                   = 360,
	SYS_bind                         = 361,
	SYS_connect                      = 362,
	SYS_listen                       = 363,
	SYS_accept4                      = 364,
	SYS_getsockopt                   = 365,
	SYS_setsockopt                   = 366,
	SYS_getsockname                  = 367,
	SYS_getpeername                  = 368,
	SYS_sendto                       = 369,
	SYS_sendmsg                      = 370,
	SYS_recvfrom                     = 371,
	SYS_recvmsg                      = 372,
	SYS_shutdown                     = 373,
	SYS_userfaultfd                  = 374,
	SYS_membarrier                   = 375,
	SYS_mlock2                       = 376,
	SYS_copy_file_range              = 377,
	SYS_preadv2                      = 378,
	SYS_pwritev2                     = 379,
	SYS_pkey_mprotect                = 380,
	SYS_pkey_alloc                   = 381,
	SYS_pkey_free                    = 382,
	SYS_statx                        = 383,
	SYS_arch_prctl                   = 384,
	SYS_io_pgetevents                = 385,
	SYS_rseq                         = 386,
	SYS_semget                       = 393,
	SYS_semctl                       = 394,
	SYS_shmget                       = 395,
	SYS_shmctl                       = 396,
	SYS_shmat                        = 397,
	SYS_shmdt                        = 398,
	SYS_msgget                       = 399,
	SYS_msgsnd                       = 400,
	SYS_msgrcv                       = 401,
	SYS_msgctl                       = 402,
	SYS_clock_gettime64              = 403,
	SYS_clock_settime64              = 404,
	SYS_clock_adjtime64              = 405,
	SYS_clock_getres_time64          = 406,
	SYS_clock_nanosleep_time64       = 407,
	SYS_timer_gettime64              = 408,
	SYS_timer_settime64              = 409,
	SYS_timerfd_gettime64            = 410,
	SYS_timerfd_settime64            = 411,
	SYS_utimensat_time64             = 412,
	SYS_pselect6_time64              = 413,
	SYS_ppoll_time64                 = 414,
	SYS_io_pgetevents_time64         = 416,
	SYS_recvmmsg_time64              = 417,
	SYS_mq_timedsend_time64          = 418,
	SYS_mq_timedreceive_time64       = 419,
	SYS_semtimedop_time64            = 420,
	SYS_rt_sigtimedwait_time64       = 421,
	SYS_futex_time64                 = 422,
	SYS_sched_rr_get_interval_time64 = 423,
	SYS_pidfd_send_signal            = 424,
	SYS_io_uring_setup               = 425,
	SYS_io_uring_enter               = 426,
	SYS_io_uring_register            = 427,
	SYS_open_tree                    = 428,
	SYS_move_mount                   = 429,
	SYS_fsopen                       = 430,
	SYS_fsconfig                     = 431,
	SYS_fsmount                      = 432,
	SYS_fspick                       = 433,
	SYS_pidfd_open                   = 434,
	SYS_clone3                       = 435,
	SYS_close_range                  = 436,
	SYS_openat2                      = 437,
	SYS_pidfd_getfd                  = 438,
	SYS_faccessat2                   = 439,
	SYS_process_madvise              = 440,
	SYS_epoll_pwait2                 = 441,
	SYS_mount_setattr                = 442,
	SYS_quotactl_fd                  = 443,
	SYS_landlock_create_ruleset      = 444,
	SYS_landlock_add_rule            = 445,
	SYS_landlock_restrict_self       = 446,
	SYS_memfd_secret                 = 447,
	SYS_process_mrelease             = 448,
	SYS_futex_waitv                  = 449,
	SYS_set_mempolicy_home_node      = 450,
}

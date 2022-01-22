macro_rules! method {
    ($i: ident) => {
        pub const $i: &'static str = stringify!($i);
    }
}

method!(GETCONF);

method!(CHECKIN);
method!(BUYCS);


method!(LOGINLIST);
method!(LCHATLIST);
method!(SETST);
method!(CHATINFO);
method!(CHATONROOM);
method!(WRITE);
method!(FORWARD);
method!(DELETEMSG);
method!(LEAVE);
method!(NOTIREAD);
method!(SETMETA);
method!(SYNCMSG);
method!(GETMEM);
method!(MEMBER);
method!(UPDATECHAT);
method!(GETTRAILER);

method!(PING);
method!(CHANGESVR);
method!(MSG);
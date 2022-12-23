use std::fmt;

pub struct Message {
    pub command: Command,
    pub params: Vec<String>,
}

pub fn parse_message(s: &str) -> Result<Message, ParseErr> {
    let trimmed = s.trim_end_matches(|c| c == '\r' || c == '\n').to_string();

    let mut i: i32 = 0;
    let mut j: i32;

    // there's this whole prefix business, but I dunno what it does, so let's eat it.
    if trimmed.starts_with(':') {
        i = (trimmed.find(' ').unwrap() + 1) as i32;
    }

    // next chunk is the command
    j = i + trimmed.get((i as usize)..).unwrap().find(' ').unwrap_or(0) as i32;
    let cmd: &str = if j > i {
        trimmed.get((i as usize)..(j as usize)).unwrap()
    } else {
        return Ok(Message {
            command: Command::from_str(trimmed.get((i as usize)..).unwrap()).unwrap(),
            params: vec![],
        });
    };

    let command = Command::from_str(cmd);
    if command.is_err() {
        return Err(command.err().unwrap());
    }

    //
    // 	// Find prefix for trailer. Note that because we need to match the trailing
    // 	// argument even if it's the only one, we can't skip the space until we've
    // 	// searched for it.
    i = trimmed
        .get((j as usize)..)
        .unwrap()
        .find(" :")
        .map(|x| x as i32)
        .unwrap_or(-1);
    //
    // 	// Skip the space
    j += 1;
    if i < 0 {
        return Ok(Message {
            command: command.unwrap(),
            params: trimmed
                .get((j as usize)..)
                .unwrap()
                .to_string()
                .split(' ')
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        });
    }

    // 	// Compensate for index on substring. Note that we skipped the space after
    // 	// looking for i, so we need to subtract 1 to account for that.
    i = i + j - 1;
    //
    // 	// Check if we need to parse arguments.
    let mut params: Vec<String> = vec![];
    if i > j {
        params = trimmed
            .get((j as usize)..(i as usize))
            .unwrap()
            .to_string()
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    }
    params.push(trimmed.get((i as usize) + 2..).unwrap().to_string());

    Ok(Message {
        command: command.unwrap(),
        params,
    })
}

#[derive(Debug, Clone)]
pub struct ParseErr {
    s: String,
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse string '{}' into IRC message", self.s)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
pub enum Command {
    PASS,
    NICK,
    USER,
    OPER,
    MODE,
    SERVICE,
    QUIT,
    SQUIT,
    JOIN,
    PART,
    TOPIC,
    NAMES,
    LIST,
    INVITE,
    KICK,
    PRIVMSG,
    NOTICE,
    MOTD,
    LUSERS,
    VERSION,
    STATS,
    LINKS,
    TIME,
    CONNECT,
    TRACE,
    ADMIN,
    INFO,
    SERVLIST,
    SQUERY,
    WHO,
    WHOIS,
    WHOWAS,
    KILL,
    PING,
    PONG,
    ERROR,
    AWAY,
    REHASH,
    DIE,
    RESTART,
    SUMMON,
    USERS,
    WALLOPS,
    USERHOST,
    ISON,
    SERVER,
    NJOIN,
    RPL_WELCOME,
    RPL_YOURHOST,
    RPL_CREATED,
    RPL_MYINFO,
    RPL_BOUNCE,
    RPL_ISUPPORT,
    RPL_USERHOST,
    RPL_ISON,
    RPL_AWAY,
    RPL_UNAWAY,
    RPL_NOWAWAY,
    RPL_WHOISUSER,
    RPL_WHOISSERVER,
    RPL_WHOISOPERATOR,
    RPL_WHOISIDLE,
    RPL_ENDOFWHOIS,
    RPL_WHOISCHANNELS,
    RPL_WHOWASUSER,
    RPL_ENDOFWHOWAS,
    RPL_LISTSTART,
    RPL_LIST,
    RPL_LISTEND,
    RPL_UNIQOPIS,
    RPL_CHANNELMODEIS,
    RPL_NOTOPIC,
    RPL_TOPIC,
    RPL_INVITING,
    RPL_SUMMONING,
    RPL_INVITELIST,
    RPL_ENDOFINVITELIST,
    RPL_EXCEPTLIST,
    RPL_ENDOFEXCEPTLIST,
    RPL_VERSION,
    RPL_WHOREPLY,
    RPL_ENDOFWHO,
    RPL_NAMREPLY,
    RPL_ENDOFNAMES,
    RPL_LINKS,
    RPL_ENDOFLINKS,
    RPL_BANLIST,
    RPL_ENDOFBANLIST,
    RPL_INFO,
    RPL_ENDOFINFO,
    RPL_MOTDSTART,
    RPL_MOTD,
    RPL_ENDOFMOTD,
    RPL_YOUREOPER,
    RPL_REHASHING,
    RPL_YOURESERVICE,
    RPL_TIME,
    RPL_USERSSTART,
    RPL_USERS,
    RPL_ENDOFUSERS,
    RPL_NOUSERS,
    RPL_TRACELINK,
    RPL_TRACECONNECTING,
    RPL_TRACEHANDSHAKE,
    RPL_TRACEUNKNOWN,
    RPL_TRACEOPERATOR,
    RPL_TRACEUSER,
    RPL_TRACESERVER,
    RPL_TRACESERVICE,
    RPL_TRACENEWTYPE,
    RPL_TRACECLASS,
    RPL_TRACERECONNECT,
    RPL_TRACELOG,
    RPL_TRACEEND,
    RPL_STATSLINKINFO,
    RPL_STATSCOMMANDS,
    RPL_ENDOFSTATS,
    RPL_STATSUPTIME,
    RPL_STATSOLINE,
    RPL_UMODEIS,
    RPL_SERVLIST,
    RPL_SERVLISTEND,
    RPL_LUSERCLIENT,
    RPL_LUSEROP,
    RPL_LUSERUNKNOWN,
    RPL_LUSERCHANNELS,
    RPL_LUSERME,
    RPL_ADMINME,
    RPL_ADMINLOC1,
    RPL_ADMINLOC2,
    RPL_ADMINEMAIL,
    RPL_TRYAGAIN,
    ERR_NOSUCHNICK,
    ERR_NOSUCHSERVER,
    ERR_NOSUCHCHANNEL,
    ERR_CANNOTSENDTOCHAN,
    ERR_TOOMANYCHANNELS,
    ERR_WASNOSUCHNICK,
    ERR_TOOMANYTARGETS,
    ERR_NOSUCHSERVICE,
    ERR_NOORIGIN,
    ERR_NORECIPIENT,
    ERR_NOTEXTTOSEND,
    ERR_NOTOPLEVEL,
    ERR_WILDTOPLEVEL,
    ERR_BADMASK,
    ERR_UNKNOWNCOMMAND,
    ERR_NOMOTD,
    ERR_NOADMININFO,
    ERR_FILEERROR,
    ERR_NONICKNAMEGIVEN,
    ERR_ERRONEUSNICKNAME,
    ERR_NICKNAMEINUSE,
    ERR_NICKCOLLISION,
    ERR_UNAVAILRESOURCE,
    ERR_USERNOTINCHANNEL,
    ERR_NOTONCHANNEL,
    ERR_USERONCHANNEL,
    ERR_NOLOGIN,
    ERR_SUMMONDISABLED,
    ERR_USERSDISABLED,
    ERR_NOTREGISTERED,
    ERR_NEEDMOREPARAMS,
    ERR_ALREADYREGISTRED,
    ERR_NOPERMFORHOST,
    ERR_PASSWDMISMATCH,
    ERR_YOUREBANNEDCREEP,
    ERR_YOUWILLBEBANNED,
    ERR_KEYSET,
    ERR_CHANNELISFULL,
    ERR_UNKNOWNMODE,
    ERR_INVITEONLYCHAN,
    ERR_BANNEDFROMCHAN,
    ERR_BADCHANNELKEY,
    ERR_BADCHANMASK,
    ERR_NOCHANMODES,
    ERR_BANLISTFULL,
    ERR_NOPRIVILEGES,
    ERR_CHANOPRIVSNEEDED,
    ERR_CANTKILLSERVER,
    ERR_RESTRICTED,
    ERR_UNIQOPPRIVSNEEDED,
    ERR_NOOPERHOST,
    ERR_UMODEUNKNOWNFLAG,
    ERR_USERSDONTMATCH,
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::PASS => "PASS",
            Command::NICK => "NICK",
            Command::USER => "USER",
            Command::OPER => "OPER",
            Command::MODE => "MODE",
            Command::SERVICE => "SERVICE",
            Command::QUIT => "QUIT",
            Command::SQUIT => "SQUIT",
            Command::JOIN => "JOIN",
            Command::PART => "PART",
            Command::TOPIC => "TOPIC",
            Command::NAMES => "NAMES",
            Command::LIST => "LIST",
            Command::INVITE => "INVITE",
            Command::KICK => "KICK",
            Command::PRIVMSG => "PRIVMSG",
            Command::NOTICE => "NOTICE",
            Command::MOTD => "MOTD",
            Command::LUSERS => "LUSERS",
            Command::VERSION => "VERSION",
            Command::STATS => "STATS",
            Command::LINKS => "LINKS",
            Command::TIME => "TIME",
            Command::CONNECT => "CONNECT",
            Command::TRACE => "TRACE",
            Command::ADMIN => "ADMIN",
            Command::INFO => "INFO",
            Command::SERVLIST => "SERVLIST",
            Command::SQUERY => "SQUERY",
            Command::WHO => "WHO",
            Command::WHOIS => "WHOIS",
            Command::WHOWAS => "WHOWAS",
            Command::KILL => "KILL",
            Command::PING => "PING",
            Command::PONG => "PONG",
            Command::ERROR => "ERROR",
            Command::AWAY => "AWAY",
            Command::REHASH => "REHASH",
            Command::DIE => "DIE",
            Command::RESTART => "RESTART",
            Command::SUMMON => "SUMMON",
            Command::USERS => "USERS",
            Command::WALLOPS => "WALLOPS",
            Command::USERHOST => "USERHOST",
            Command::ISON => "ISON",
            Command::SERVER => "SERVER",
            Command::NJOIN => "NJOIN",
            Command::RPL_WELCOME => "001",
            Command::RPL_YOURHOST => "002",
            Command::RPL_CREATED => "003",
            Command::RPL_MYINFO => "004",
            Command::RPL_BOUNCE => "010",
            Command::RPL_ISUPPORT => "005",
            Command::RPL_USERHOST => "302",
            Command::RPL_ISON => "303",
            Command::RPL_AWAY => "301",
            Command::RPL_UNAWAY => "305",
            Command::RPL_NOWAWAY => "306",
            Command::RPL_WHOISUSER => "311",
            Command::RPL_WHOISSERVER => "312",
            Command::RPL_WHOISOPERATOR => "313",
            Command::RPL_WHOISIDLE => "317",
            Command::RPL_ENDOFWHOIS => "318",
            Command::RPL_WHOISCHANNELS => "319",
            Command::RPL_WHOWASUSER => "314",
            Command::RPL_ENDOFWHOWAS => "369",
            Command::RPL_LISTSTART => "321",
            Command::RPL_LIST => "322",
            Command::RPL_LISTEND => "323",
            Command::RPL_UNIQOPIS => "325",
            Command::RPL_CHANNELMODEIS => "324",
            Command::RPL_NOTOPIC => "331",
            Command::RPL_TOPIC => "332",
            Command::RPL_INVITING => "341",
            Command::RPL_SUMMONING => "342",
            Command::RPL_INVITELIST => "346",
            Command::RPL_ENDOFINVITELIST => "347",
            Command::RPL_EXCEPTLIST => "348",
            Command::RPL_ENDOFEXCEPTLIST => "349",
            Command::RPL_VERSION => "351",
            Command::RPL_WHOREPLY => "352",
            Command::RPL_ENDOFWHO => "315",
            Command::RPL_NAMREPLY => "353",
            Command::RPL_ENDOFNAMES => "366",
            Command::RPL_LINKS => "364",
            Command::RPL_ENDOFLINKS => "365",
            Command::RPL_BANLIST => "367",
            Command::RPL_ENDOFBANLIST => "368",
            Command::RPL_INFO => "371",
            Command::RPL_ENDOFINFO => "374",
            Command::RPL_MOTDSTART => "375",
            Command::RPL_MOTD => "372",
            Command::RPL_ENDOFMOTD => "376",
            Command::RPL_YOUREOPER => "381",
            Command::RPL_REHASHING => "382",
            Command::RPL_YOURESERVICE => "383",
            Command::RPL_TIME => "391",
            Command::RPL_USERSSTART => "392",
            Command::RPL_USERS => "393",
            Command::RPL_ENDOFUSERS => "394",
            Command::RPL_NOUSERS => "395",
            Command::RPL_TRACELINK => "200",
            Command::RPL_TRACECONNECTING => "201",
            Command::RPL_TRACEHANDSHAKE => "202",
            Command::RPL_TRACEUNKNOWN => "203",
            Command::RPL_TRACEOPERATOR => "204",
            Command::RPL_TRACEUSER => "205",
            Command::RPL_TRACESERVER => "206",
            Command::RPL_TRACESERVICE => "207",
            Command::RPL_TRACENEWTYPE => "208",
            Command::RPL_TRACECLASS => "209",
            Command::RPL_TRACERECONNECT => "210",
            Command::RPL_TRACELOG => "261",
            Command::RPL_TRACEEND => "262",
            Command::RPL_STATSLINKINFO => "211",
            Command::RPL_STATSCOMMANDS => "212",
            Command::RPL_ENDOFSTATS => "219",
            Command::RPL_STATSUPTIME => "242",
            Command::RPL_STATSOLINE => "243",
            Command::RPL_UMODEIS => "221",
            Command::RPL_SERVLIST => "234",
            Command::RPL_SERVLISTEND => "235",
            Command::RPL_LUSERCLIENT => "251",
            Command::RPL_LUSEROP => "252",
            Command::RPL_LUSERUNKNOWN => "253",
            Command::RPL_LUSERCHANNELS => "254",
            Command::RPL_LUSERME => "255",
            Command::RPL_ADMINME => "256",
            Command::RPL_ADMINLOC1 => "257",
            Command::RPL_ADMINLOC2 => "258",
            Command::RPL_ADMINEMAIL => "259",
            Command::RPL_TRYAGAIN => "263",
            Command::ERR_NOSUCHNICK => "401",
            Command::ERR_NOSUCHSERVER => "402",
            Command::ERR_NOSUCHCHANNEL => "403",
            Command::ERR_CANNOTSENDTOCHAN => "404",
            Command::ERR_TOOMANYCHANNELS => "405",
            Command::ERR_WASNOSUCHNICK => "406",
            Command::ERR_TOOMANYTARGETS => "407",
            Command::ERR_NOSUCHSERVICE => "408",
            Command::ERR_NOORIGIN => "409",
            Command::ERR_NORECIPIENT => "411",
            Command::ERR_NOTEXTTOSEND => "412",
            Command::ERR_NOTOPLEVEL => "413",
            Command::ERR_WILDTOPLEVEL => "414",
            Command::ERR_BADMASK => "415",
            Command::ERR_UNKNOWNCOMMAND => "421",
            Command::ERR_NOMOTD => "422",
            Command::ERR_NOADMININFO => "423",
            Command::ERR_FILEERROR => "424",
            Command::ERR_NONICKNAMEGIVEN => "431",
            Command::ERR_ERRONEUSNICKNAME => "432",
            Command::ERR_NICKNAMEINUSE => "433",
            Command::ERR_NICKCOLLISION => "436",
            Command::ERR_UNAVAILRESOURCE => "437",
            Command::ERR_USERNOTINCHANNEL => "441",
            Command::ERR_NOTONCHANNEL => "442",
            Command::ERR_USERONCHANNEL => "443",
            Command::ERR_NOLOGIN => "444",
            Command::ERR_SUMMONDISABLED => "445",
            Command::ERR_USERSDISABLED => "446",
            Command::ERR_NOTREGISTERED => "451",
            Command::ERR_NEEDMOREPARAMS => "461",
            Command::ERR_ALREADYREGISTRED => "462",
            Command::ERR_NOPERMFORHOST => "463",
            Command::ERR_PASSWDMISMATCH => "464",
            Command::ERR_YOUREBANNEDCREEP => "465",
            Command::ERR_YOUWILLBEBANNED => "466",
            Command::ERR_KEYSET => "467",
            Command::ERR_CHANNELISFULL => "471",
            Command::ERR_UNKNOWNMODE => "472",
            Command::ERR_INVITEONLYCHAN => "473",
            Command::ERR_BANNEDFROMCHAN => "474",
            Command::ERR_BADCHANNELKEY => "475",
            Command::ERR_BADCHANMASK => "476",
            Command::ERR_NOCHANMODES => "477",
            Command::ERR_BANLISTFULL => "478",
            Command::ERR_NOPRIVILEGES => "481",
            Command::ERR_CHANOPRIVSNEEDED => "482",
            Command::ERR_CANTKILLSERVER => "483",
            Command::ERR_RESTRICTED => "484",
            Command::ERR_UNIQOPPRIVSNEEDED => "485",
            Command::ERR_NOOPERHOST => "491",
            Command::ERR_UMODEUNKNOWNFLAG => "501",
            Command::ERR_USERSDONTMATCH => "502",
        }
    }

    fn from_str(s: &str) -> Result<Self, ParseErr> {
        match s {
            "PASS" => Ok(Command::PASS),
            "NICK" => Ok(Command::NICK),
            "USER" => Ok(Command::USER),
            "OPER" => Ok(Command::OPER),
            "MODE" => Ok(Command::MODE),
            "SERVICE" => Ok(Command::SERVICE),
            "QUIT" => Ok(Command::QUIT),
            "SQUIT" => Ok(Command::SQUIT),
            "JOIN" => Ok(Command::JOIN),
            "PART" => Ok(Command::PART),
            "TOPIC" => Ok(Command::TOPIC),
            "NAMES" => Ok(Command::NAMES),
            "LIST" => Ok(Command::LIST),
            "INVITE" => Ok(Command::INVITE),
            "KICK" => Ok(Command::KICK),
            "PRIVMSG" => Ok(Command::PRIVMSG),
            "NOTICE" => Ok(Command::NOTICE),
            "MOTD" => Ok(Command::MOTD),
            "LUSERS" => Ok(Command::LUSERS),
            "VERSION" => Ok(Command::VERSION),
            "STATS" => Ok(Command::STATS),
            "LINKS" => Ok(Command::LINKS),
            "TIME" => Ok(Command::TIME),
            "CONNECT" => Ok(Command::CONNECT),
            "TRACE" => Ok(Command::TRACE),
            "ADMIN" => Ok(Command::ADMIN),
            "INFO" => Ok(Command::INFO),
            "SERVLIST" => Ok(Command::SERVLIST),
            "SQUERY" => Ok(Command::SQUERY),
            "WHO" => Ok(Command::WHO),
            "WHOIS" => Ok(Command::WHOIS),
            "WHOWAS" => Ok(Command::WHOWAS),
            "KILL" => Ok(Command::KILL),
            "PING" => Ok(Command::PING),
            "PONG" => Ok(Command::PONG),
            "ERROR" => Ok(Command::ERROR),
            "AWAY" => Ok(Command::AWAY),
            "REHASH" => Ok(Command::REHASH),
            "DIE" => Ok(Command::DIE),
            "RESTART" => Ok(Command::RESTART),
            "SUMMON" => Ok(Command::SUMMON),
            "USERS" => Ok(Command::USERS),
            "WALLOPS" => Ok(Command::WALLOPS),
            "USERHOST" => Ok(Command::USERHOST),
            "ISON" => Ok(Command::ISON),
            "SERVER" => Ok(Command::SERVER),
            "NJOIN" => Ok(Command::NJOIN),
            "001" => Ok(Command::RPL_WELCOME),
            "002" => Ok(Command::RPL_YOURHOST),
            "003" => Ok(Command::RPL_CREATED),
            "004" => Ok(Command::RPL_MYINFO),
            "010" => Ok(Command::RPL_BOUNCE),
            "005" => Ok(Command::RPL_ISUPPORT),
            "302" => Ok(Command::RPL_USERHOST),
            "303" => Ok(Command::RPL_ISON),
            "301" => Ok(Command::RPL_AWAY),
            "305" => Ok(Command::RPL_UNAWAY),
            "306" => Ok(Command::RPL_NOWAWAY),
            "311" => Ok(Command::RPL_WHOISUSER),
            "312" => Ok(Command::RPL_WHOISSERVER),
            "313" => Ok(Command::RPL_WHOISOPERATOR),
            "317" => Ok(Command::RPL_WHOISIDLE),
            "318" => Ok(Command::RPL_ENDOFWHOIS),
            "319" => Ok(Command::RPL_WHOISCHANNELS),
            "314" => Ok(Command::RPL_WHOWASUSER),
            "369" => Ok(Command::RPL_ENDOFWHOWAS),
            "321" => Ok(Command::RPL_LISTSTART),
            "322" => Ok(Command::RPL_LIST),
            "323" => Ok(Command::RPL_LISTEND),
            "325" => Ok(Command::RPL_UNIQOPIS),
            "324" => Ok(Command::RPL_CHANNELMODEIS),
            "331" => Ok(Command::RPL_NOTOPIC),
            "332" => Ok(Command::RPL_TOPIC),
            "341" => Ok(Command::RPL_INVITING),
            "342" => Ok(Command::RPL_SUMMONING),
            "346" => Ok(Command::RPL_INVITELIST),
            "347" => Ok(Command::RPL_ENDOFINVITELIST),
            "348" => Ok(Command::RPL_EXCEPTLIST),
            "349" => Ok(Command::RPL_ENDOFEXCEPTLIST),
            "351" => Ok(Command::RPL_VERSION),
            "352" => Ok(Command::RPL_WHOREPLY),
            "315" => Ok(Command::RPL_ENDOFWHO),
            "353" => Ok(Command::RPL_NAMREPLY),
            "366" => Ok(Command::RPL_ENDOFNAMES),
            "364" => Ok(Command::RPL_LINKS),
            "365" => Ok(Command::RPL_ENDOFLINKS),
            "367" => Ok(Command::RPL_BANLIST),
            "368" => Ok(Command::RPL_ENDOFBANLIST),
            "371" => Ok(Command::RPL_INFO),
            "374" => Ok(Command::RPL_ENDOFINFO),
            "375" => Ok(Command::RPL_MOTDSTART),
            "372" => Ok(Command::RPL_MOTD),
            "376" => Ok(Command::RPL_ENDOFMOTD),
            "381" => Ok(Command::RPL_YOUREOPER),
            "382" => Ok(Command::RPL_REHASHING),
            "383" => Ok(Command::RPL_YOURESERVICE),
            "391" => Ok(Command::RPL_TIME),
            "392" => Ok(Command::RPL_USERSSTART),
            "393" => Ok(Command::RPL_USERS),
            "394" => Ok(Command::RPL_ENDOFUSERS),
            "395" => Ok(Command::RPL_NOUSERS),
            "200" => Ok(Command::RPL_TRACELINK),
            "201" => Ok(Command::RPL_TRACECONNECTING),
            "202" => Ok(Command::RPL_TRACEHANDSHAKE),
            "203" => Ok(Command::RPL_TRACEUNKNOWN),
            "204" => Ok(Command::RPL_TRACEOPERATOR),
            "205" => Ok(Command::RPL_TRACEUSER),
            "206" => Ok(Command::RPL_TRACESERVER),
            "207" => Ok(Command::RPL_TRACESERVICE),
            "208" => Ok(Command::RPL_TRACENEWTYPE),
            "209" => Ok(Command::RPL_TRACECLASS),
            "210" => Ok(Command::RPL_TRACERECONNECT),
            "261" => Ok(Command::RPL_TRACELOG),
            "262" => Ok(Command::RPL_TRACEEND),
            "211" => Ok(Command::RPL_STATSLINKINFO),
            "212" => Ok(Command::RPL_STATSCOMMANDS),
            "219" => Ok(Command::RPL_ENDOFSTATS),
            "242" => Ok(Command::RPL_STATSUPTIME),
            "243" => Ok(Command::RPL_STATSOLINE),
            "221" => Ok(Command::RPL_UMODEIS),
            "234" => Ok(Command::RPL_SERVLIST),
            "235" => Ok(Command::RPL_SERVLISTEND),
            "251" => Ok(Command::RPL_LUSERCLIENT),
            "252" => Ok(Command::RPL_LUSEROP),
            "253" => Ok(Command::RPL_LUSERUNKNOWN),
            "254" => Ok(Command::RPL_LUSERCHANNELS),
            "255" => Ok(Command::RPL_LUSERME),
            "256" => Ok(Command::RPL_ADMINME),
            "257" => Ok(Command::RPL_ADMINLOC1),
            "258" => Ok(Command::RPL_ADMINLOC2),
            "259" => Ok(Command::RPL_ADMINEMAIL),
            "263" => Ok(Command::RPL_TRYAGAIN),
            "401" => Ok(Command::ERR_NOSUCHNICK),
            "402" => Ok(Command::ERR_NOSUCHSERVER),
            "403" => Ok(Command::ERR_NOSUCHCHANNEL),
            "404" => Ok(Command::ERR_CANNOTSENDTOCHAN),
            "405" => Ok(Command::ERR_TOOMANYCHANNELS),
            "406" => Ok(Command::ERR_WASNOSUCHNICK),
            "407" => Ok(Command::ERR_TOOMANYTARGETS),
            "408" => Ok(Command::ERR_NOSUCHSERVICE),
            "409" => Ok(Command::ERR_NOORIGIN),
            "411" => Ok(Command::ERR_NORECIPIENT),
            "412" => Ok(Command::ERR_NOTEXTTOSEND),
            "413" => Ok(Command::ERR_NOTOPLEVEL),
            "414" => Ok(Command::ERR_WILDTOPLEVEL),
            "415" => Ok(Command::ERR_BADMASK),
            "421" => Ok(Command::ERR_UNKNOWNCOMMAND),
            "422" => Ok(Command::ERR_NOMOTD),
            "423" => Ok(Command::ERR_NOADMININFO),
            "424" => Ok(Command::ERR_FILEERROR),
            "431" => Ok(Command::ERR_NONICKNAMEGIVEN),
            "432" => Ok(Command::ERR_ERRONEUSNICKNAME),
            "433" => Ok(Command::ERR_NICKNAMEINUSE),
            "436" => Ok(Command::ERR_NICKCOLLISION),
            "437" => Ok(Command::ERR_UNAVAILRESOURCE),
            "441" => Ok(Command::ERR_USERNOTINCHANNEL),
            "442" => Ok(Command::ERR_NOTONCHANNEL),
            "443" => Ok(Command::ERR_USERONCHANNEL),
            "444" => Ok(Command::ERR_NOLOGIN),
            "445" => Ok(Command::ERR_SUMMONDISABLED),
            "446" => Ok(Command::ERR_USERSDISABLED),
            "451" => Ok(Command::ERR_NOTREGISTERED),
            "461" => Ok(Command::ERR_NEEDMOREPARAMS),
            "462" => Ok(Command::ERR_ALREADYREGISTRED),
            "463" => Ok(Command::ERR_NOPERMFORHOST),
            "464" => Ok(Command::ERR_PASSWDMISMATCH),
            "465" => Ok(Command::ERR_YOUREBANNEDCREEP),
            "466" => Ok(Command::ERR_YOUWILLBEBANNED),
            "467" => Ok(Command::ERR_KEYSET),
            "471" => Ok(Command::ERR_CHANNELISFULL),
            "472" => Ok(Command::ERR_UNKNOWNMODE),
            "473" => Ok(Command::ERR_INVITEONLYCHAN),
            "474" => Ok(Command::ERR_BANNEDFROMCHAN),
            "475" => Ok(Command::ERR_BADCHANNELKEY),
            "476" => Ok(Command::ERR_BADCHANMASK),
            "477" => Ok(Command::ERR_NOCHANMODES),
            "478" => Ok(Command::ERR_BANLISTFULL),
            "481" => Ok(Command::ERR_NOPRIVILEGES),
            "482" => Ok(Command::ERR_CHANOPRIVSNEEDED),
            "483" => Ok(Command::ERR_CANTKILLSERVER),
            "484" => Ok(Command::ERR_RESTRICTED),
            "485" => Ok(Command::ERR_UNIQOPPRIVSNEEDED),
            "491" => Ok(Command::ERR_NOOPERHOST),
            "501" => Ok(Command::ERR_UMODEUNKNOWNFLAG),
            "502" => Ok(Command::ERR_USERSDONTMATCH),
            _ => Err(ParseErr { s: s.to_string() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let result = parse_message(":foo!~bar@baz.com PRIVMSG #qux :!ping");
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.command == Command::PRIVMSG);
        assert_eq!(msg.params, vec!["#qux", "!ping"]);
    }
}
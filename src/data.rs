static BOOTLOG: &str = include_str!("../data/bootlog.txt");
static CFILES: &str = include_str!("../data/cfiles.txt");
static PACKAGES: &str = include_str!("../data/packages.txt");
static COMPOSERS: &str = include_str!("../data/composer.txt");
static SIMCITY: &str = include_str!("../data/simcity.txt");
static BOOT_HOOKS: &str = include_str!("../data/boot_hooks.txt");
static OS_RELEASES: &str = include_str!("../data/os_releases.txt");
static DOCKER_PACKAGES: &str = include_str!("../data/docker_packages.txt");
static DOCKER_TAGS: &str = include_str!("../data/docker_tags.txt");
static ANSIBLE_ROLES: &str = include_str!("../data/ansible_roles.txt");
static ANSIBLE_TASKS: &str = include_str!("../data/ansible_tasks.txt");
static RKHUNTER_CHECKS: &str = include_str!("../data/rkhunter_checks.txt");
static RKHUNTER_ROOTKITS: &str = include_str!("../data/rkhunter_rootkits.txt");
static RKHUNTER_TASKS: &str = include_str!("../data/rkhunter_tasks.txt");
static JULIA_PACKAGES: &str = include_str!("../data/julia.csv");

lazy_static::lazy_static! {
    pub static ref BOOTLOG_LIST: Vec<&'static str> = BOOTLOG.lines().collect();
    pub static ref CFILES_LIST: Vec<&'static str> = CFILES.lines().collect();
    pub static ref PACKAGES_LIST: Vec<&'static str> = PACKAGES.lines().collect();
    pub static ref COMPOSERS_LIST: Vec<&'static str> = COMPOSERS.lines().collect();
    pub static ref SIMCITY_LIST: Vec<&'static str> = SIMCITY.lines().collect();
    pub static ref BOOT_HOOKS_LIST: Vec<&'static str> = BOOT_HOOKS.lines().collect();
    pub static ref OS_RELEASES_LIST: Vec<&'static str> = OS_RELEASES.lines().collect();
    pub static ref DOCKER_PACKAGES_LIST: Vec<&'static str> = DOCKER_PACKAGES.lines().collect();
    pub static ref DOCKER_TAGS_LIST: Vec<&'static str> = DOCKER_TAGS.lines().collect();
    pub static ref ANSIBLE_ROLES_LIST: Vec<&'static str> = ANSIBLE_ROLES.lines().collect();
    pub static ref ANSIBLE_TASKS_LIST: Vec<&'static str> = ANSIBLE_TASKS.lines().collect();
    pub static ref RKHUNTER_CHECKS_LIST: Vec<&'static str> = RKHUNTER_CHECKS.lines().collect();
    pub static ref RKHUNTER_ROOTKITS_LIST: Vec<&'static str> = RKHUNTER_ROOTKITS.lines().collect();
    pub static ref RKHUNTER_TASKS_LIST: Vec<&'static str> = RKHUNTER_TASKS.lines().collect();
    pub static ref JULIA_PACKAGES_LIST: Vec<crate::modules::julia::Package<'static>> = JULIA_PACKAGES
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let name = split.next().unwrap_or("Revise");
            let id = split.next().unwrap_or("295af30f");
            let versions = split.collect();
            crate::modules::julia::Package { name, id, versions }
        })
        .collect();

}

pub static EXTENSIONS_LIST: &[&str] = &[
    "gif", "mkv", "webm", "mp4", "html", "php", "md", "png", "jpg", "opus", "ogg", "mp3", "flac",
    "iso", "zip", "rar", "tar.gz", "tar.bz2", "tar.xz", "tar.zstd", "deb", "rpm", "exe",
];

pub static COMPRESSION_FORMATS_LIST: &[&str] =
    &["gzip", "bzip2", "lzma", "xz", "lzop", "lz4", "zstd"];

pub static PASSWORDS_AND_HASHES_LIST: &[(&str, &str)] = &[
    (
        "password",
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8",
    ),
    (
        "welcome",
        "280d44ab1e9f79b5cce2dd4f58f5fe91f0fbacdac9f7447dffc318ceb79f2d02",
    ),
    (
        "qwerty",
        "65e84be33532fb784c48129675f9eff3a682b27168c0ea744b2cf58ee02337c5",
    ),
    (
        "monkey",
        "000c285457fc971f862a79b786476c78812c8897063c6fa9c045f579a3b2d63f",
    ),
    (
        "jesus",
        "a54e71f0e17f5aaf7946e66ab42cf3b1fd4e61d60581736c9f0eb1c3f794eb7c",
    ),
    (
        "love",
        "686f746a95b6f836d7d70567c302c3f9ebb5ee0def3d1220ee9d4e9f34f5e131",
    ),
    (
        "money",
        "8d2ac8b58ead9744d77286de9b0bcb7a894f238c3149fc9f3b1e3caff36330fe",
    ),
    (
        "freedom",
        "13b1f7ec5beaefc781e43a3b344371cd49923a8a05edd71844b92f56f6a08d38",
    ),
    (
        "ninja",
        "54482595177116e6103b076dbf30648e5d0537dd1ed9cf5ae4562fa8a700d47b",
    ),
];

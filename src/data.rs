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
}

pub static EXTENSIONS_LIST: &[&str] = &[
    "gif", "mkv", "webm", "mp4", "html", "php", "md", "png", "jpg", "opus", "ogg", "mp3", "flac",
    "iso", "zip", "rar", "tar.gz", "tar.bz2", "tar.xz", "tar.zstd", "deb", "rpm", "exe",
];

pub static COMPRESSION_FORMATS_LIST: &[&str] =
    &["gzip", "bzip2", "lzma", "xz", "lzop", "lz4", "zstd"];

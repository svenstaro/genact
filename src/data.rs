use std::sync::LazyLock;

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
static TERRAFORM_AWS_RESOURCES: &str = include_str!("../data/terraform_aws_resources.txt");
static TERRAFORM_AZURE_RESOURCES: &str = include_str!("../data/terraform_azure_resources.txt");
static TERRAFORM_GCP_RESOURCES: &str = include_str!("../data/terraform_gcp_resources.txt");
static TERRAFORM_IDS: &str = include_str!("../data/terraform_ids.txt");
static CSS_PROPERTIES: &str = include_str!("../data/css_properties.txt");
static WEB_APIS: &str = include_str!("../data/web_apis.txt");
static WPT_CATEGORIES: &str = include_str!("../data/wpt_categories.txt");

pub static BOOTLOG_LIST: LazyLock<Vec<&'static str>> = LazyLock::new(|| BOOTLOG.lines().collect());
pub static CFILES_LIST: LazyLock<Vec<&'static str>> = LazyLock::new(|| CFILES.lines().collect());
pub static PACKAGES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| PACKAGES.lines().collect());
pub static COMPOSERS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| COMPOSERS.lines().collect());
pub static SIMCITY_LIST: LazyLock<Vec<&'static str>> = LazyLock::new(|| SIMCITY.lines().collect());
pub static BOOT_HOOKS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| BOOT_HOOKS.lines().collect());
pub static OS_RELEASES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| OS_RELEASES.lines().collect());
pub static DOCKER_PACKAGES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| DOCKER_PACKAGES.lines().collect());
pub static DOCKER_TAGS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| DOCKER_TAGS.lines().collect());
pub static ANSIBLE_ROLES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| ANSIBLE_ROLES.lines().collect());
pub static ANSIBLE_TASKS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| ANSIBLE_TASKS.lines().collect());
pub static RKHUNTER_CHECKS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| RKHUNTER_CHECKS.lines().collect());
pub static RKHUNTER_ROOTKITS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| RKHUNTER_ROOTKITS.lines().collect());
pub static RKHUNTER_TASKS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| RKHUNTER_TASKS.lines().collect());
pub static TERRAFORM_AWS_RESOURCES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| TERRAFORM_AWS_RESOURCES.lines().collect());
pub static TERRAFORM_AZURE_RESOURCES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| TERRAFORM_AZURE_RESOURCES.lines().collect());
pub static TERRAFORM_GCP_RESOURCES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| TERRAFORM_GCP_RESOURCES.lines().collect());
pub static TERRAFORM_IDS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| TERRAFORM_IDS.lines().collect());
pub static JULIA_PACKAGES_LIST: LazyLock<Vec<crate::modules::julia::Package<'static>>> =
    LazyLock::new(|| {
        JULIA_PACKAGES
            .lines()
            .map(|line| {
                let mut split = line.split(',');
                let name = split.next().unwrap_or("Revise");
                let id = split.next().unwrap_or("295af30f");
                let versions = split.collect();
                crate::modules::julia::Package { name, id, versions }
            })
            .collect()
    });
pub static CSS_PROPERTIES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| CSS_PROPERTIES.lines().collect());
pub static WEB_APIS_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| WEB_APIS.lines().collect());
pub static WPT_CATEGORIES_LIST: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| WPT_CATEGORIES.lines().collect());

pub static EXTENSIONS_LIST: &[&str] = &[
    "gif", "mkv", "webm", "mp4", "html", "php", "md", "png", "jpg", "opus", "ogg", "mp3", "flac",
    "iso", "zip", "rar", "tar.gz", "tar.bz2", "tar.xz", "tar.zstd", "deb", "rpm", "exe",
];

pub static COMPRESSION_FORMATS_LIST: &[&str] =
    &["gzip", "bzip2", "lzma", "xz", "lzop", "lz4", "zstd"];

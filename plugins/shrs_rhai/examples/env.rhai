let path = env("PATH");
let pwd = env("PWD");

print(`PATH: ${path}`);
print(`PWD: ${pwd}`);

export_env("MY_VALUE", "1");
let val = env("MY_VALUE");
print(`MY_VALUE: ${val}`);
export_env("MY_VALUE", "2");
let val = env("MY_VALUE");
print(`MY_VALUE: ${val}`);

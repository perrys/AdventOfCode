
def all_binaries(bins):
    for file in bins:
        base = file.replace(".cpp", "")
        native.cc_binary(
            name = base,
            srcs = [file],
            deps = ["//lib:scp", "//lib:nr"],
        )

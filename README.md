# InfiniTime resource loader

Loads resources into an InfiniTime device.
<https://github.com/InfiniTimeOrg/InfiniTime/blob/develop/doc/ExternalResources.md>

Find the resource archive under the released assets on the InfiniTime
repo, <https://github.com/InfiniTimeOrg/InfiniTime/releases>.
It's called something like `infinitime-resources-$version.zip`.
Then flash to your device with `infinitime-resource-loader flash $path`.

## Usage
```bash
$ ./infinitime-resource-loader -h
Usage: infinitime-resource-loader <COMMAND>

Commands:
  flash  Flashes contents of a resource archive unto the device
  list   List files on the device
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```bash
$ ./infinitime-resource-loader flash -h
Flashes contents of a resource archive unto the device

Usage: infinitime-resource-loader flash <resource-path>

Arguments:
  <resource-path>  Path for the resources archive

Options:
  -h, --help  Print help
```

```bash
$ ./infinitime-resource-loader list -h
List files on the device

Usage: infinitime-resource-loader list <PATH>

Arguments:
  <PATH>  Path to list

Options:
  -h, --help  Print help
```

## Examples
```bash
$ RUST_LOG=warn ./infinitime-resource-loader flash infinitime-resources-1.11.0.zip
[2023-02-22T14:09:40Z WARN  infinitime_resource_loader] Error when trying to make directory: Received error from device: Unknown error: 0xef. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.
[21s] ████████████████████████████████████████ 4.81 KiB/4.81 KiB 7segments_115.bin sent.                                                                                                        [2023-02-22T14:10:02Z WARN  infinitime_resource_loader] Error when trying to make directory: Received error from device: Unknown error: 0xef. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.
[5s] ████████████████████████████████████████ 1.80 KiB/1.80 KiB lv_font_dots_40.bin sent.                                                                                                       [2023-02-22T14:10:07Z WARN  infinitime_resource_loader] Error when trying to make directory: Received error from device: Unknown error: 0xef. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.
[2s] ████████████████████████████████████████    764B/764B    teko.bin sent.                                                                                                                    [2023-02-22T14:10:09Z WARN  infinitime_resource_loader] Error when trying to make directory: Received error from device: Unknown error: 0xef. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.
[19s] ████████████████████████████████████████ 4.32 KiB/4.32 KiB bebas.bin sent.                                                                                                                [2023-02-22T14:10:28Z WARN  infinitime_resource_loader] Error when trying to make directory: Received error from device: Unknown error: 0xef. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.
[2s] ████████████████████████████████████████    760B/760B    7segments_40.bin sent.                                                                                                            [2023-02-22T14:10:30Z WARN  infinitime_resource_loader] Error when trying to make directory: Received error from device: Unknown error: 0xef. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.
[6s] ████████████████████████████████████████ 2.09 KiB/2.09 KiB pine_small.bin sent.
```

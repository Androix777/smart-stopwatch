# Smart Stopwatch

A smart stopwatch that automatically turns on/off when certain windows are in focus.

The stopwatch is configured by drag-and-drop the settings yaml file into the stopwatch window. This system allows you to keep different settings for different usage scenarios. The settings specify the windows at which the stopwatch counts time. Example of a settings file "settings.yaml":

```yaml
whitelist:
- title: 'Window name in regex format (windows such as browser change their name depending on what is open inside).'
  process_path: 'Path to the process file in regex format'
  app_name: 'Application name in regex format'
  window_id: 'Window ID in regex format'
  process_id: 'Process ID as a number'
- title: 'Parameters for the second window'
  process_path: 'Parameters for the second window'
- title: 'Parameters for the third window'
```

But usually it is enough just to specify one or two fields for each window. For example, these settings enable the stopwatch when Google is open in Firefox or when Discord is open:

```yaml
whitelist:
- title: 'Google Search — Mozilla Firefox'
- app_name: '^Discord$'
```
^ and $ in this case are part of the regex syntax, meaning that the name of the application must match the data completely, not just include it as a substring.

You can check the window parameters by pressing the "C" button at the top of the stopwatch while the required window is in focus. Then the parameters of the current window in json format will be copied to the clipboard. For example:

```json
{
  "title":"Friends - Discord",
  "process_path":"C:\\Users\\qwerty\\AppData\\Local\\Discord\\app-1.0.9037\\Discord.exe",
  "app_name":"Discord",
  "window_id":"HWND(66844)",
  "process_id":20172,
  "position":{"x":-8,"y":-8,"width":2576,"height":1416}
}
```

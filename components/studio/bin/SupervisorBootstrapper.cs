using System;
using System.Diagnostics;
using System.IO;

public class SupervisorBootstrapper {
    private static String logPath = Path.Combine(Environment.GetEnvironmentVariable("HAB_STUDIO_ENTER_ROOT"), "hab\\sup\\default\\out.log");

    public static void Run(bool isAnsiSupported) {
        var proc = new Process();
        proc.StartInfo.UseShellExecute = false;
        proc.StartInfo.CreateNoWindow = true;
        proc.StartInfo.RedirectStandardOutput = true;
        proc.StartInfo.RedirectStandardError = true;
        proc.StartInfo.FileName = "hab.exe";
        if (isAnsiSupported) {
            proc.StartInfo.Arguments = "sup run";
        }
        else {
            proc.StartInfo.Arguments = "sup run --no-color";
            proc.StartInfo.EnvironmentVariables["HAB_NOCOLORING"] = "1";
        }
        proc.OutputDataReceived += new DataReceivedEventHandler(SupOutputHandler);
        proc.ErrorDataReceived += new DataReceivedEventHandler(SupOutputHandler);
        proc.Start();
        proc.BeginErrorReadLine();
        proc.BeginOutputReadLine();
    }

    private static void SupOutputHandler(object sender, DataReceivedEventArgs e) {
        if (!String.IsNullOrEmpty(e.Data)) {
            using (FileStream fs = new FileStream(logPath, FileMode.Append, FileAccess.Write, FileShare.ReadWrite)) {
                using (StreamWriter sw = new StreamWriter(fs, System.Text.Encoding.UTF8)) {
                    sw.WriteLine(e.Data);
                }
            }
        }
    }
}

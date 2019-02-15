using System;
using System.Configuration;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using System.ServiceProcess;
using System.Xml;

namespace HabService
{
    public partial class HabService : ServiceBase
    {
        [DllImport("kernel32.dll", SetLastError = true)]
        static extern bool AttachConsole(uint dwProcessId);

        [DllImport("kernel32.dll", SetLastError = true, ExactSpelling = true)]
        static extern bool FreeConsole();

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool GenerateConsoleCtrlEvent(CtrlTypes dwCtrlEvent, uint dwProcessGroupId);

        [DllImport("Kernel32.dll", SetLastError = true)]
        private static extern bool SetConsoleCtrlHandler(HandlerRoutine handler, bool add);

        private delegate bool HandlerRoutine(CtrlTypes CtrlType);

        // Enumerated type for the control messages sent to the handler routine
        enum CtrlTypes
        {
            CTRL_C_EVENT = 0,
            CTRL_BREAK_EVENT,
            CTRL_CLOSE_EVENT,
            CTRL_LOGOFF_EVENT = 5,
            CTRL_SHUTDOWN_EVENT
        }

        private Process proc = null;
        private static readonly log4net.ILog log = log4net.LogManager.GetLogger(typeof(HabService));

        /// <summary>
        /// The main entry point for the service.
        /// </summary>
        static void Main()
        {
            var log4netConfig = new XmlDocument();
            var codebase = Assembly.GetExecutingAssembly().CodeBase;
            var uri = new UriBuilder(codebase);
            var path = Uri.UnescapeDataString(uri.Path);
            log4netConfig.Load(File.OpenRead(Path.Join(Path.GetDirectoryName(path), "log4net.xml")));

            var repo = log4net.LogManager.CreateRepository(
                Assembly.GetEntryAssembly(), typeof(log4net.Repository.Hierarchy.Hierarchy));

            log4net.Config.XmlConfigurator.Configure(repo, log4netConfig["log4net"]);

            ServiceBase[] ServicesToRun;
            ServicesToRun = new ServiceBase[]
            {
                new HabService()
            };
            Run(ServicesToRun);
        }

        public HabService()
        {
            ServiceName = "HabService";
            CanStop = true;
            AutoLog = true;
        }

        protected override void OnStart(string[] args)
        {
            try
            {
                ConfigureDebug();
                ConfigureSupSignal();
                proc = new Process();
                proc.StartInfo.UseShellExecute = false;
                proc.StartInfo.CreateNoWindow = true;
                proc.StartInfo.RedirectStandardOutput = true;
                proc.StartInfo.RedirectStandardError = true;
                proc.StartInfo.FileName = LauncherPath;
                string launcherArgs = "run";
                if (ConfigurationManager.AppSettings["launcherArgs"] != null)
                {
                    launcherArgs += String.Format(" {0}", ConfigurationManager.AppSettings["launcherArgs"]);
                }
                proc.StartInfo.Arguments = launcherArgs;
                log.Info(String.Format("Habitat windows service is starting launcher at: {0}", LauncherPath));
                log.Info(String.Format("Habitat windows service is starting launcher with args: {0}", launcherArgs));
                proc.OutputDataReceived += new DataReceivedEventHandler(SupOutputHandler);
                proc.ErrorDataReceived += new DataReceivedEventHandler(SupErrorHandler);
                proc.Start();
                proc.BeginErrorReadLine();
                proc.BeginOutputReadLine();
            }
            catch(Exception e)
            {
                log.Error("Error occured in OnStart", e);
            }
        }

        private static void ConfigureDebug()
        {
            if (ConfigurationManager.AppSettings["debug"] != null)
            {
                if (ConfigurationManager.AppSettings["debug"].ToLower() != "false")
                {
                    Environment.SetEnvironmentVariable("RUST_LOG", "debug");
                }
                else
                {
                    Environment.SetEnvironmentVariable("RUST_LOG", null);
                }
            }
            else
            {
                Environment.SetEnvironmentVariable("RUST_LOG", null);
            }
        }

        private static void ConfigureSupSignal()
        {
            if (ConfigurationManager.AppSettings["HAB_FEAT_IGNORE_SIGNALS"] != null)
            {
                if (ConfigurationManager.AppSettings["HAB_FEAT_IGNORE_SIGNALS"].ToLower() != "false")
                {
                    Environment.SetEnvironmentVariable("HAB_FEAT_IGNORE_SIGNALS", "true");
                }
                else
                {
                    Environment.SetEnvironmentVariable("HAB_FEAT_IGNORE_SIGNALS", null);
                }
            }
            else
            {
                Environment.SetEnvironmentVariable("HAB_FEAT_IGNORE_SIGNALS", null);
            }
        }

        private static string LauncherPath
        {
            get
            {
                if (ConfigurationManager.AppSettings["launcherPath"] != null)
                {
                    return ConfigurationManager.AppSettings["launcherPath"];
                }
                else
                {
                    // because we declare hab-launcher as a runtime dep
                    // this path should exist
                    string launcherBase = Path.Combine(Path.GetPathRoot(Environment.SystemDirectory), "hab\\pkgs\\core\\hab-launcher");
                    string latestLauncher = LastDirectory(LastDirectory(launcherBase));
                    return Path.Combine(latestLauncher, "bin\\hab-launch.exe");
                }
            }
        }

        private static string LastDirectory(string path)
        {
            string[] dirs = Directory.GetDirectories(path);
            return dirs[dirs.Length - 1];
        }

        private void SupOutputHandler(object sender, DataReceivedEventArgs e)
        {
            if (!String.IsNullOrEmpty(e.Data))
            {
                log.Info(e.Data);
            }
        }

        private void SupErrorHandler(object sender, DataReceivedEventArgs e)
        {
            if (!String.IsNullOrEmpty(e.Data))
            {
                log.Error(e.Data);
            }
        }

        protected override void OnStop()
        {
            try
            {
                // As a service we have no console so attach to the console of the launcher
                if(!AttachConsole((uint)proc.Id)) {
                    log.Error("Unable to attach to console!");
                    log.Error(Marshal.GetLastWin32Error());
                }
                // Turn off our own Ctrl-C handler so we don't die
                if(!SetConsoleCtrlHandler(null, true)) {
                    log.Error("Failed to disable ctrl+c!");
                    log.Error(Marshal.GetLastWin32Error());
                }
                // Broadcast the ctrl-c
                if(!GenerateConsoleCtrlEvent(CtrlTypes.CTRL_C_EVENT, 0)) {
                    log.Error("Failed to send ctrl+c signal!");
                    log.Error(Marshal.GetLastWin32Error());
                }

                if (!proc.WaitForExit(60000))
                {
                    log.Error("Launcher did not exit after waiting for one minute!");
                    log.Info("Forcefully terminating Launcher process.");
                    proc.Kill();
                }

                // Remove ourselves from the dead console
                FreeConsole();

                log.Info("Habitat service stopped");
            }
            catch(Exception e)
            {
                log.Error("Error occured in OnStop", e);
            }
        }
    }
}

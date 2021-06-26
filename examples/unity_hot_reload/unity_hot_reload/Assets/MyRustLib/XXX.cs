using System.IO;
using UnityEditor;
using UnityEngine;

namespace Interoptopus.Util
{
    class HotReload
    {
        private const string DllSource = @"";
        private const string DllDst = @"%DLL_DESTINATION%";
        private const string InteropDstRoot = @"Assets/";
        private const string InteropSrcRoot = @"";
        private static readonly string[] InteropFiles = {
            %INTEROP_FILES%
        };

        static void UpdateInteropFiles()
        {
            Debug.Log("Running");
            File.Copy(DllSource, DllDst);

            foreach (var file in InteropFiles)
            {

                File.Copy(Path.Combine(InteropSrcRoot, file), Path.Combine(InteropDstRoot, file));
            }
        }

        [InitializeOnLoadMethod]
        static void OnProjectLoadedInEditor()
        {
            Debug.Log("Project loaded in Unity Editor 2");


        }

        [MenuItem("Interoptopus/Hot Reload - %PROJECT_NAME%")]
        static void Init()
        {
            UpdateInteropFiles();
        }
    }

}

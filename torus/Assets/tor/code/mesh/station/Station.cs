
using UnityEngine;
using System.Collections;
using System.Runtime.InteropServices;

public class Station : MonoBehaviour {

    // The plugin referenced by `DLLImport` must be compiled
    // and placed in your project's Assets/Plugins/ folder.

    // Make sure that you rename the lib<name>.dylib to
    // lib<name>.bundle so that Unity can find it.

    [DllImport("station")]
    private static extern int double_input(int x);

    [DllImport("station")]
    private static extern int mesh_double_input(int x);

    void Start() {
        Debug.Log(double_input(2)); // 4
        Debug.Log(mesh_double_input(4)); // 8
    }
}
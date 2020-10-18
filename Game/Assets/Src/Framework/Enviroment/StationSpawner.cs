using UnityEngine;
//using Unity.Entities;
using Unity.Transforms;
using Unity.Rendering;
using Unity.Mathematics;
//using Unity.Physics;
public class StationSpawner : MonoBehaviour
{
    [SerializeField] private Mesh moduleMesh;// these are enterable things
    [SerializeField] private Material moduleMat;



    // Start is called before the first frame update
    void Start()
    {
        MakeEntity();
    }

    private void MakeEntity()
    {
        

        for (int i = 0; i < 18; i++)
        {


            Vector3 trans = new Vector3(0f, 0f, 0f);
            Quaternion angle = Quaternion.Euler(360 / 18 * i, 0f, 0f);
 


            GameObject newMod = new GameObject("module" + i);
            newMod.AddComponent<Rigidbody>();
            newMod.AddComponent<MeshFilter>();
            newMod.AddComponent<MeshRenderer>();
            newMod.AddComponent<MeshCollider>();
            newMod.transform.position = trans;
            newMod.transform.rotation = angle;

            newMod.GetComponent<MeshFilter>().mesh = moduleMesh;
            newMod.GetComponent<MeshRenderer>().material = moduleMat;
            newMod.GetComponent<MeshCollider>().sharedMesh = moduleMesh;
            newMod.GetComponent<Rigidbody>().isKinematic = true;
            newMod.GetComponent<Rigidbody>().useGravity = false;


        }
    }
}

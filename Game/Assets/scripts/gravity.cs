using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Unity.Mathematics;
public class gravity : MonoBehaviour
{

    public Rigidbody rb;
    public float forceMultiplier;
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        Vector3 position = transform.position;
        float diffy = position.y;
        float diffz = position.z;
        float angle = math.atan(diffy / diffz);
        float y = math.sin(angle);
        float z = math.cos(angle);
        Vector3 newForce = new Vector3(0, y * forceMultiplier, z * forceMultiplier);


        rb = GetComponent<Rigidbody>();

        rb.AddForce(newForce, ForceMode.Acceleration);



    }



}

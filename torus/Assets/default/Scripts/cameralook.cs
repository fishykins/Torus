using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
public class cameralook : MonoBehaviour
{
    // Start is called before the first frame update

    public float speedH = 2.0f;
    public float speedV = 2.0f;
    public Rigidbody RB;

    private float yaw = 0.0f;
    private float pitch = 0.0f;


    void Start()
    {
        Cursor.lockState = CursorLockMode.Locked;
    }

    // Update is called once per frame
    void Update()
    {
        float speed = Convert.ToSingle(0.1);

        yaw += speedH * Input.GetAxis("Mouse X");
        pitch -= speedV * Input.GetAxis("Mouse Y");

        transform.eulerAngles = new Vector3(pitch, yaw, 0.0f);
    }
}

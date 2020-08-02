using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace Tor.Meshes
{
    struct MeshData
    {
        public Vector3[] vertices;
        public Color32[] colors;
        public Vector3[] normals;
        public Vector2[] uv;
        public Vector2[] uv2;


        public MeshData(Vector3[] vertices, Color32[] colors, Vector3[] normals, Vector2[] uv, Vector2[] uv2)
        {
            this.vertices = vertices;
            this.colors = colors;
            this.normals = normals;
            this.uv = uv;
            this.uv2 = uv2;
        }

        public MeshData(Vector3[] vertices, Vector3[] normals, Vector2[] uv)
        {
            this.vertices = vertices;
            this.normals = normals;
            this.uv = uv;
            this.colors = new Color32[1089];
            this.uv2 = new Vector2[1089];

        }
    }
}
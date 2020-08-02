using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Tor.DataTypes;

//TODO:Please tidy this up- looks like its become bloated and can utilize the new "MeshData" struct
namespace Tor.Meshes {
	public class MeshBuilder  {
        #region Variables
        private List<Vector3> vertices = new List<Vector3>();
        private List<Vector3> normals = new List<Vector3>();
        private List<Vector2> uvs = new List<Vector2>();
        private List<int> indices = new List<int>();
        #endregion

        #region Properties
        public List<Vector3> Vertices { get { return vertices; } }
        public List<Vector3> Normals { get { return normals; } }
        public List<Vector2> UVs { get { return uvs; } }
        #endregion

        #region Methods
        public void AddTriangle(int index0, int index1, int index2)
        {
            indices.Add(index0);
            indices.Add(index1);
            indices.Add(index2);
        }

        public Mesh CreateMesh()
        {
            Mesh mesh = new Mesh();

            mesh.vertices = vertices.ToArray();
            mesh.triangles = indices.ToArray();

            //Normals are optional. Only use them if we have the correct amount:
            if (normals.Count == vertices.Count)
                mesh.normals = normals.ToArray();

            //UVs are optional. Only use them if we have the correct amount:
            if (uvs.Count == vertices.Count)
                mesh.uv = uvs.ToArray();

            mesh.RecalculateBounds();

            return mesh;
        }

        public void AddQuad(Quad quad)
        {
            AddQuad(quad.a, quad.b, quad.c, quad.d);
        }

        public void AddQuad(Vector3 a, Vector3 b, Vector3 c, Vector3 d)
        {
            Vector3 width = d - a;
            Vector3 length = b - a;
            Vector3 normal = Vector3.Cross(length, width).normalized;

            float uvScale = width.magnitude;

            Vertices.Add(a);
            UVs.Add(new Vector2(0.0f, 0.0f));
            Normals.Add(normal);

            Vertices.Add(b);
            UVs.Add(new Vector2(0.0f, uvScale));
            Normals.Add(normal);

            Vertices.Add(c);
            UVs.Add(new Vector2(uvScale, uvScale));
            Normals.Add(normal);

            Vertices.Add(d);
            UVs.Add(new Vector2(uvScale, 0.0f));
            Normals.Add(normal);

            int baseIndex = Vertices.Count - 4;

            AddTriangle(baseIndex, baseIndex + 1, baseIndex + 2);
            AddTriangle(baseIndex, baseIndex + 2, baseIndex + 3);
        }
        #endregion
    }
}
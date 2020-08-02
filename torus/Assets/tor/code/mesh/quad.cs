using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace Tor.DataTypes {
	public struct Quad  {
        public Vector3 a;
        public Vector3 b;
        public Vector3 c;
        public Vector3 d;

        public Quad(Vector3 a, Vector3 b, Vector3 c, Vector3 d)
        {
            this.a = a;
            this.b = b;
            this.c = c;
            this.d = d;
        }

        public void OffsetQuad(Vector3 offset)
        {
            a += offset;
            b += offset;
            c += offset;
            d += offset;
        }
    }
}
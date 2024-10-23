
/*! Create an icosphere

Algorithm taken from <http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html>
*/

use std::collections::HashMap;

use crate::vectors::Vector;

//public class IcoSphereCreator
//{
//	private struct TriangleIndices
//	{
//		public int v1;
//		public int v2;
//		public int v3;
//
//		public TriangleIndices(int v1, int v2, int v3)
//		{
//			this.v1 = v1;
//			this.v2 = v2;
//			this.v3 = v3;
//		}
//	}
//
//	private MeshGeometry3D geometry;
//	private int index;
//	private Dictionary<Int64, int> middlePointIndexCache;


// add vertex to mesh, fix position to be on unit sphere, return index
fn add_vertex(vertices: &mut Vec::<[f64; 3]>, x: f64, y: f64, z: f64) {
	let l = (x * x + y * y + z * z).sqrt();
	vertices.push([x/l, y/l, z/l]);
//		return index++;
}

pub fn get_normals(vertices: &Vec::<[f64; 3]>, faces: &Vec::<[usize; 3]>) -> Vec::<[f64; 3]> {
	let mut out = Vec::new();

	for f in faces {
		let p0 = vertices[f[0]];
		let p1 = vertices[f[1]];
		let p2 = vertices[f[2]];

		// This is the proper way, except it doesn't check what direction the normals are pointing
		let v0 = Vector::new(p0[0], p0[1], p0[2]);
		let v1 = Vector::new(p1[0], p1[1], p1[2]);
		let v2 = Vector::new(p2[0], p2[1], p2[2]);
		let v = (v1-v0).cross(&(v2-v0)).norm();

		out.push([v.x, v.y, v.z]);
		// This is the hacky way, that assumes the faces are fairly small
//		out.push([(p0[0] + p1[0] + p2[0])/3.0,
//		          (p0[1] + p1[1] + p2[1])/3.0,
//		          (p0[2] + p1[2] + p2[2])/3.0]);

	}
	out
}

// return index of point in the middle of p1 and p2
fn get_middle_point(vertices: &mut Vec::<[f64; 3]>, cache: &mut HashMap::<i64, usize>, p1: usize, p2: usize) -> usize {
	let first_is_smaller = p1 < p2;
	let smaller: i64 = if first_is_smaller { p1 } else { p2 } as i64;
	let greater: i64 = if first_is_smaller { p2 } else { p1 } as i64;
	let key: i64 = (smaller << 32) + greater;

    match cache.get(&key) {
        Some(val) => return *val,
        None => {}
    }
	
	// not in cache
	add_vertex(vertices, (vertices[p1][0] + vertices[p2][0]) / 2.0,
	                          (vertices[p1][1] + vertices[p2][1]) / 2.0,
	                          (vertices[p1][2] + vertices[p2][2]) / 2.0);
	
	let l = vertices.len() - 1;
	cache.insert(key, l);
	l
}
//	private int getMiddlePoint(int p1, int p2)
//	{
//		// first check if we have it already
//		bool firstIsSmaller = p1 < p2;
//		Int64 smallerIndex = firstIsSmaller ? p1 : p2;
//		Int64 greaterIndex = firstIsSmaller ? p2 : p1;
//		Int64 key = (smallerIndex << 32) + greaterIndex;
//
//		int ret;
//		if (this.middlePointIndexCache.TryGetValue(key, out ret))
//		{
//			return ret;
//		}
//
//		// not in cache, calculate it
//		Point3D point1 = this.geometry.Positions[p1];
//		Point3D point2 = this.geometry.Positions[p2];
//		Point3D middle = new Point3D(
//			(point1.X + point2.X) / 2.0,
//			(point1.Y + point2.Y) / 2.0,
//			(point1.Z + point2.Z) / 2.0);
//
//		// add vertex makes sure point is on unit sphere
//		int i = addVertex(middle);
//
//		// store it, return index
//		this.middlePointIndexCache.Add(key, i);
//		return i;
//	}

pub fn create_icosphere(recursion_level: usize) -> (Vec::<[f64; 3]>, Vec::<[usize; 3]>) {
	let mut vertices = Vec::<[f64; 3]>::new();
	let mut faces = Vec::<[usize; 3]>::new();

	let t = (1.0 + 5_f64.sqrt()) / 2.0;

// create 12 vertices of icosahedron
	add_vertex(&mut vertices, -1.0, t, 0.0);
	add_vertex(&mut vertices,  1.0, t, 0.0);
	add_vertex(&mut vertices, -1.0,-t, 0.0);
	add_vertex(&mut vertices,  1.0,-t, 0.0);

	add_vertex(&mut vertices, 0.0,-1.0, t);
	add_vertex(&mut vertices, 0.0, 1.0, t);
	add_vertex(&mut vertices, 0.0,-1.0,-t);
	add_vertex(&mut vertices, 0.0, 1.0,-t);

	add_vertex(&mut vertices,  t, 0.0,-1.0);
	add_vertex(&mut vertices,  t, 0.0, 1.0);
	add_vertex(&mut vertices, -t, 0.0,-1.0);
	add_vertex(&mut vertices, -t, 0.0, 1.0);

// create 20 triangles of the icosahedron
	// 5 faces around point 0
	faces.push([0, 11, 5]);
	faces.push([0, 5,  1]);
	faces.push([0, 1,  7]);
	faces.push([0, 7, 10]);
	faces.push([0, 10,11]);

	// 5 adjacent faces
	faces.push([1,  5, 9]);
	faces.push([5,  11,4]);
	faces.push([11, 10,2]);
	faces.push([10, 7, 6]);
	faces.push([7,  1, 8]);

	// 5 faces around point 3
	faces.push([3, 9, 4]);
	faces.push([3, 4, 2]);
	faces.push([3, 2, 6]);
	faces.push([3, 6, 8]);
	faces.push([3, 8, 9]);

	// 5 adjacent faces
	faces.push([4, 9,  5]);
	faces.push([2, 4, 11]);
	faces.push([6, 2, 10]);
	faces.push([8, 6,  7]);
	faces.push([9, 8,  1]);
	

// refine triangles
	let mut cache = HashMap::<i64, usize>::new();
	for _i in 0..recursion_level {
		let mut faces_temp = Vec::new();
		for f in faces {
			// replace triangle by 4 triangles
			let a = get_middle_point(&mut vertices, &mut cache, f[0], f[1]);
			let b = get_middle_point(&mut vertices, &mut cache, f[1], f[2]);
			let c = get_middle_point(&mut vertices, &mut cache, f[2], f[0]);

			faces_temp.push([f[0], a, c]);
			faces_temp.push([f[1], b, a]);
			faces_temp.push([f[2], c, b]);
			faces_temp.push([a, b, c]);
		}
		faces = faces_temp;
	}

	(vertices, faces)
//K3dMesh::new(Geometry {
//		vertices: &vertices,
//		faces: &faces,
//		colors: &[],
//		lines: &[],
//		normals: &[],
//	})
}



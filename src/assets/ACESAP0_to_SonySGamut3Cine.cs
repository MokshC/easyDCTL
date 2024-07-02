//
//  ACES AP0 to Sony S-Gamut3.Cine
//

__DEVICE__ float3 AcesAp0ToSonySGamut3Cine(float3 rgb)
{	
    float3 result;
    
	result.x = rgb.x * 1.55546f + rgb.y * -0.393281f + rgb.z * -0.162178f;
	result.y = rgb.x * 0.0090216f + rgb.y * 0.918557f + rgb.z * 0.0724214f;
	result.z = rgb.x * 0.0442641f + rgb.y * 0.0118503f + rgb.z * 0.943886f;
    
    return result;
}


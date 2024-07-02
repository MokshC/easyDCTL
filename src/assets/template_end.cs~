//
//  This DCTL is made by easyDCTL
//	https://github.com/MokshC/easydctl
//


__DEVICE__ float3 transform(int p_Width, int p_Height, int p_X, int p_Y, float p_R, float p_G, float p_B)
{
	
	float3 rgb = make_float3(p_R, p_G, p_B);
	
	float3 rgblin = LogToLin(rgb);
	float3 rgbaces = ColorToAces(rgblin);
	float3 rgbcolor = AcesToColor(rgbaces);
	float3 rgblog = LinToLog(rgbcolor);
	
	float3 result = [replace];

    return result;
}


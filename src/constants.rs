extern crate xiapi_sys;
extern crate libc;

use libc::{c_void,c_char};
use std::mem;
use std::ffi::CString;
use std::convert::Into;

pub enum ModelID
{
UNKNOWN = xiapi_sys::xiapi::MODEL_ID_UNKNOWN,
MR274CU_BH = xiapi_sys::xiapi::MODEL_ID_MR274CU_BH,
MR16000MU = xiapi_sys::xiapi::MODEL_ID_MR16000MU,
MR282CC_BH = xiapi_sys::xiapi::MODEL_ID_MR282CC_BH,
MR274MU_BH = xiapi_sys::xiapi::MODEL_ID_MR274MU_BH,
MR456CU_BH = xiapi_sys::xiapi::MODEL_ID_MR456CU_BH,
MR252CC_BH = xiapi_sys::xiapi::MODEL_ID_MR252CC_BH,
MR4021MU_BH = xiapi_sys::xiapi::MODEL_ID_MR4021MU_BH,
MR4022MU_BH = xiapi_sys::xiapi::MODEL_ID_MR4022MU_BH,
MR655CU_BH = xiapi_sys::xiapi::MODEL_ID_MR655CU_BH,
MR11002M = xiapi_sys::xiapi::MODEL_ID_MR11002M,
MR4021CU_BH = xiapi_sys::xiapi::MODEL_ID_MR4021CU_BH,
MR655MU_BH = xiapi_sys::xiapi::MODEL_ID_MR655MU_BH,
MR282CU_BH = xiapi_sys::xiapi::MODEL_ID_MR282CU_BH,
MR252CU_BH = xiapi_sys::xiapi::MODEL_ID_MR252CU_BH,
MR285MU_BH = xiapi_sys::xiapi::MODEL_ID_MR285MU_BH,
MR285CU_BH = xiapi_sys::xiapi::MODEL_ID_MR285CU_BH,
MR285MC_BH = xiapi_sys::xiapi::MODEL_ID_MR285MC_BH,
MR285CC_BH = xiapi_sys::xiapi::MODEL_ID_MR285CC_BH,
MH160MC_KK_FA = xiapi_sys::xiapi::MODEL_ID_MH160MC_KK_FA,
MU9PC_BH = xiapi_sys::xiapi::MODEL_ID_MU9PC_BH,
MR11002C = xiapi_sys::xiapi::MODEL_ID_MR11002C,
MU9PM_MH = xiapi_sys::xiapi::MODEL_ID_MU9PM_MH,
MU9PC_MH = xiapi_sys::xiapi::MODEL_ID_MU9PC_MH,
MU9PM_BH = xiapi_sys::xiapi::MODEL_ID_MU9PM_BH,
CURRERA_RS04 = xiapi_sys::xiapi::MODEL_ID_CURRERA_RS04,
CURRERA_RL04 = xiapi_sys::xiapi::MODEL_ID_CURRERA_RL04,
CURRERA_RS04C = xiapi_sys::xiapi::MODEL_ID_CURRERA_RS04C,
CURRERA_RL04C = xiapi_sys::xiapi::MODEL_ID_CURRERA_RL04C,
CURRERA_RS13 = xiapi_sys::xiapi::MODEL_ID_CURRERA_RS13,
CURRERA_RL13 = xiapi_sys::xiapi::MODEL_ID_CURRERA_RL13,
CURRERA_RS13C = xiapi_sys::xiapi::MODEL_ID_CURRERA_RS13C,
CURRERA_RL13C = xiapi_sys::xiapi::MODEL_ID_CURRERA_RL13C,
CURRERA_RS50 = xiapi_sys::xiapi::MODEL_ID_CURRERA_RS50,
CURRERA_RL50 = xiapi_sys::xiapi::MODEL_ID_CURRERA_RL50,
CURRERA_RS50C = xiapi_sys::xiapi::MODEL_ID_CURRERA_RS50C,
MQ013CG_E2 = xiapi_sys::xiapi::MODEL_ID_MQ013CG_E2,
MQ013MG_E2 = xiapi_sys::xiapi::MODEL_ID_MQ013MG_E2,
MQ003CG_CM = xiapi_sys::xiapi::MODEL_ID_MQ003CG_CM,
MQ003MG_CM = xiapi_sys::xiapi::MODEL_ID_MQ003MG_CM,
MQ022CG_CM = xiapi_sys::xiapi::MODEL_ID_MQ022CG_CM,
MQ022MG_CM = xiapi_sys::xiapi::MODEL_ID_MQ022MG_CM,
MQ042CG_CM = xiapi_sys::xiapi::MODEL_ID_MQ042CG_CM,
MQ042MG_CM = xiapi_sys::xiapi::MODEL_ID_MQ042MG_CM,
MQ022MG_CM_SR2 = xiapi_sys::xiapi::MODEL_ID_MQ022MG_CM_SR2,
MQ042CG_CM_TG = xiapi_sys::xiapi::MODEL_ID_MQ042CG_CM_TG,
MQ042MG_CM_TG = xiapi_sys::xiapi::MODEL_ID_MQ042MG_CM_TG,
MQ_USB3LINK = xiapi_sys::xiapi::MODEL_ID_MQ_USB3LINK,
MU9PC_SLC5 = xiapi_sys::xiapi::MODEL_ID_MU9PC_SLC5,
MQ022CG_CM_TS = xiapi_sys::xiapi::MODEL_ID_MQ022CG_CM_TS,
MQ022MG_CM_TS = xiapi_sys::xiapi::MODEL_ID_MQ022MG_CM_TS,
MQ042CG_CM_TS = xiapi_sys::xiapi::MODEL_ID_MQ042CG_CM_TS,
MQ042MG_CM_TS = xiapi_sys::xiapi::MODEL_ID_MQ042MG_CM_TS,
MQ013CG_ONV = xiapi_sys::xiapi::MODEL_ID_MQ013CG_ONV,
MQ013MG_ONV = xiapi_sys::xiapi::MODEL_ID_MQ013MG_ONV,
MQ013RG_E2 = xiapi_sys::xiapi::MODEL_ID_MQ013RG_E2,
MQ042RG_CM = xiapi_sys::xiapi::MODEL_ID_MQ042RG_CM,
CURRERA_RL50C = xiapi_sys::xiapi::MODEL_ID_CURRERA_RL50C,
MR11002XC_ICW = xiapi_sys::xiapi::MODEL_ID_MR11002XC_ICW,
MQ020CG_E2 = xiapi_sys::xiapi::MODEL_ID_MQ020CG_E2,
MQ020MG_E2 = xiapi_sys::xiapi::MODEL_ID_MQ020MG_E2,
MQ022RG_CM = xiapi_sys::xiapi::MODEL_ID_MQ022RG_CM,
MR285CC_DP = xiapi_sys::xiapi::MODEL_ID_MR285CC_DP,
MR285MC_DP = xiapi_sys::xiapi::MODEL_ID_MR285MC_DP,
MR252CU_BRD = xiapi_sys::xiapi::MODEL_ID_MR252CU_BRD,
MH110MC_KK_FA = xiapi_sys::xiapi::MODEL_ID_MH110MC_KK_FA,
MR282CU_BRD = xiapi_sys::xiapi::MODEL_ID_MR282CU_BRD,
MR282CC_DP = xiapi_sys::xiapi::MODEL_ID_MR282CC_DP,
MR285MU_BH_IRE = xiapi_sys::xiapi::MODEL_ID_MR285MU_BH_IRE,
MR285MC_DP_IRE = xiapi_sys::xiapi::MODEL_ID_MR285MC_DP_IRE,
MH110XC_KK_FA = xiapi_sys::xiapi::MODEL_ID_MH110XC_KK_FA,
MH160XC_KK_FA = xiapi_sys::xiapi::MODEL_ID_MH160XC_KK_FA,
MR252CC_DP = xiapi_sys::xiapi::MODEL_ID_MR252CC_DP,
MR285MC_BH_IRE = xiapi_sys::xiapi::MODEL_ID_MR285MC_BH_IRE,
MR456CC_BH = xiapi_sys::xiapi::MODEL_ID_MR456CC_BH,
MR282CU_DP = xiapi_sys::xiapi::MODEL_ID_MR282CU_DP,
MQ022HG_IM_ST32_600_1000 = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_ST32_600_1000,
MR282CC_BRD = xiapi_sys::xiapi::MODEL_ID_MR282CC_BRD,
MR252CC_BRD = xiapi_sys::xiapi::MODEL_ID_MR252CC_BRD,
MQ022HG_IM_SM4X4_470_620 = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_SM4X4_470_620,
MR252CU_DP = xiapi_sys::xiapi::MODEL_ID_MR252CU_DP,
MR285MU_BRD = xiapi_sys::xiapi::MODEL_ID_MR285MU_BRD,
MR285CU_BRD = xiapi_sys::xiapi::MODEL_ID_MR285CU_BRD,
MR285MC_BRD = xiapi_sys::xiapi::MODEL_ID_MR285MC_BRD,
MR285CC_BRD = xiapi_sys::xiapi::MODEL_ID_MR285CC_BRD,
MR285CC_DP_IRE = xiapi_sys::xiapi::MODEL_ID_MR285CC_DP_IRE,
MR285CC_BH_IRE = xiapi_sys::xiapi::MODEL_ID_MR285CC_BH_IRE,
MR285CU_BH_IRE = xiapi_sys::xiapi::MODEL_ID_MR285CU_BH_IRE,
MX11002 = xiapi_sys::xiapi::MODEL_ID_MX11002,
MH110CC_KK_FA = xiapi_sys::xiapi::MODEL_ID_MH110CC_KK_FA,
MR16000CU = xiapi_sys::xiapi::MODEL_ID_MR16000CU,
MH160CC_KK_FA = xiapi_sys::xiapi::MODEL_ID_MH160CC_KK_FA,
MR4022MC_VELETA = xiapi_sys::xiapi::MODEL_ID_MR4022MC_VELETA,
MR4021MC_VELETA = xiapi_sys::xiapi::MODEL_ID_MR4021MC_VELETA,
MU9JC_BH = xiapi_sys::xiapi::MODEL_ID_MU9JC_BH,
MU9JM_BH = xiapi_sys::xiapi::MODEL_ID_MU9JM_BH,
MQ022HG_IM_LS100_600_1000 = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_LS100_600_1000,
MD028CC_SY = xiapi_sys::xiapi::MODEL_ID_MD028CC_SY,
MD061CC_SY = xiapi_sys::xiapi::MODEL_ID_MD061CC_SY,
MD091CC_SY = xiapi_sys::xiapi::MODEL_ID_MD091CC_SY,
MD028MC_SY = xiapi_sys::xiapi::MODEL_ID_MD028MC_SY,
MD061MC_SY = xiapi_sys::xiapi::MODEL_ID_MD061MC_SY,
MD091MC_SY = xiapi_sys::xiapi::MODEL_ID_MD091MC_SY,
MD028CU_SY = xiapi_sys::xiapi::MODEL_ID_MD028CU_SY,
MD061CU_SY = xiapi_sys::xiapi::MODEL_ID_MD061CU_SY,
MD091CU_SY = xiapi_sys::xiapi::MODEL_ID_MD091CU_SY,
MD028MU_SY = xiapi_sys::xiapi::MODEL_ID_MD028MU_SY,
MD061MU_SY = xiapi_sys::xiapi::MODEL_ID_MD061MU_SY,
MD091MU_SY = xiapi_sys::xiapi::MODEL_ID_MD091MU_SY,
CB200CG_CM = xiapi_sys::xiapi::MODEL_ID_CB200CG_CM,
CB200MG_CM = xiapi_sys::xiapi::MODEL_ID_CB200MG_CM,
MD120CC_SY = xiapi_sys::xiapi::MODEL_ID_MD120CC_SY,
MD120MC_SY = xiapi_sys::xiapi::MODEL_ID_MD120MC_SY,
MD120CU_SY = xiapi_sys::xiapi::MODEL_ID_MD120CU_SY,
MD120MU_SY = xiapi_sys::xiapi::MODEL_ID_MD120MU_SY,
MQ022HG_IM_UN = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_UN,
CAL_Simulator = xiapi_sys::xiapi::MODEL_ID_CAL_Simulator,
MQ022HG_IM_LS150_470_900 = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_LS150_470_900,
MQ022HG_IM_SM5X5_600_1000 = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_SM5X5_600_1000,
MQ022HG_IM_SM4X4_600_900 = xiapi_sys::xiapi::MODEL_ID_MQ022HG_IM_SM4X4_600_900,
MQ022MG_CM_BARE_BRD = xiapi_sys::xiapi::MODEL_ID_MQ022MG_CM_BARE_BRD,
MQ042MG_CM_BARE_BRD = xiapi_sys::xiapi::MODEL_ID_MQ042MG_CM_BARE_BRD,
MT023CG_SY = xiapi_sys::xiapi::MODEL_ID_MT023CG_SY,
MT023MG_SY = xiapi_sys::xiapi::MODEL_ID_MT023MG_SY,
MT200CG_CM = xiapi_sys::xiapi::MODEL_ID_MT200CG_CM,
MT200MG_CM = xiapi_sys::xiapi::MODEL_ID_MT200MG_CM,
CB120CG_CM = xiapi_sys::xiapi::MODEL_ID_CB120CG_CM,
CB120MG_CM = xiapi_sys::xiapi::MODEL_ID_CB120MG_CM,
MT003CG_LX = xiapi_sys::xiapi::MODEL_ID_MT003CG_LX,
MT003MG_LX = xiapi_sys::xiapi::MODEL_ID_MT003MG_LX,
MQ013CG_ON = xiapi_sys::xiapi::MODEL_ID_MQ013CG_ON,
MQ013MG_ON = xiapi_sys::xiapi::MODEL_ID_MQ013MG_ON,

}
pub enum GenTLImageFormat {

Mono8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono8,
BGRA8Packed 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BGRA8Packed,
RGB8Planar 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_RGB8Planar,
BayerRG8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG8,
Mono10 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono10,
Mono12 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono12,
Mono14 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono14,
BayerRG10 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG10,
BayerRG12 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG12,
BayerGR8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR8,
BayerGB8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB8,
BayerGR10 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR10,
BayerGB10 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB10,
BayerGR12 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR12,
BayerBG8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG8,
BayerBG10 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG10,
BayerBG12 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG12,
BayerGB12 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB12,
RGB8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_RGB8,
BGR8 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BGR8,
BayerRG14 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG14,
BayerGR14 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR14,
BayerBG14 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG14,
BayerGB14 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB14,
BayerBG10p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG10p,
BayerGB10p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB10p,
BayerGR10p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR10p,
BayerRG10p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG10p,
Mono10p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono10p,
BayerBG12p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG12p,
BayerGB12p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB12p,
BayerGR12p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR12p,
BayerRG12p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG12p,
Mono12p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono12p,
BayerBG14p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerBG14p,
BayerGB14p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGB14p,
BayerGR14p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerGR14p,
BayerRG14p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_BayerRG14p,
Mono14p 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_Mono14p,
xiBayerBG10g160 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG10g160,
xiBayerGB10g160 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB10g160,
xiBayerGR10g160 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR10g160,
xiBayerRG10g160 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG10g160,
xiMono10g160 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono10g160,
xiBayerBG12g192 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG12g192,
xiBayerGB12g192 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB12g192,
xiBayerGR12g192 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR12g192,
xiBayerRG12g192 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG12g192,
xiMono12g192 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono12g192,
xiBayerBG14g224 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG14g224,
xiBayerGB14g224 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB14g224,
xiBayerGR14g224 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR14g224,
xiBayerRG14g224 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG14g224,
xiMono14g224 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono14g224,
xiMono8TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono8TS01,
xiMono10TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono10TS01,
xiMono12TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono12TS01,
xiMono14TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono14TS01,
xiBayerRG8TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG8TS01,
xiBayerRG10TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG10TS01,
xiBayerRG12TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG12TS01,
xiBayerRG14TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG14TS01,
xiBayerBG8TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG8TS01,
xiBayerBG10TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG10TS01,
xiBayerBG12TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG12TS01,
xiBayerBG14TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG14TS01,
xiBayerGB8TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB8TS01,
xiBayerGB10TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB10TS01,
xiBayerGB12TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB12TS01,
xiBayerGB14TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB14TS01,
xiBayerGR8TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR8TS01,
xiBayerGR10TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR10TS01,
xiBayerGR12TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR12TS01,
xiBayerGR14TS01 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR14TS01,
xiMono8TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono8TS03,
xiMono10TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono10TS03,
xiMono12TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono12TS03,
xiMono14TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiMono14TS03,
xiBayerRG8TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG8TS03,
xiBayerRG10TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG10TS03,
xiBayerRG12TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG12TS03,
xiBayerRG14TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerRG14TS03,
xiBayerBG8TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG8TS03,
xiBayerBG10TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG10TS03,
xiBayerBG12TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG12TS03,
xiBayerBG14TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerBG14TS03,
xiBayerGB8TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB8TS03,
xiBayerGB10TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB10TS03,
xiBayerGB12TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB12TS03,
xiBayerGB14TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGB14TS03,
xiBayerGR8TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR8TS03,
xiBayerGR10TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR10TS03,
xiBayerGR12TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR12TS03,
xiBayerGR14TS03 	=	 xiapi_sys::xiapi::XI_GenTL_Image_Format_xiBayerGR14TS03,

}

pub enum xiReturn {
OK 	=	 xiapi_sys::xiapi::XI_OK,
INVALID_HANDLE 	=	 xiapi_sys::xiapi::XI_INVALID_HANDLE,
READREG 	=	 xiapi_sys::xiapi::XI_READREG,
WRITEREG 	=	 xiapi_sys::xiapi::XI_WRITEREG,
FREE_RESOURCES 	=	 xiapi_sys::xiapi::XI_FREE_RESOURCES,
FREE_CHANNEL 	=	 xiapi_sys::xiapi::XI_FREE_CHANNEL,
FREE_BANDWIDTH 	=	 xiapi_sys::xiapi::XI_FREE_BANDWIDTH,
READBLK 	=	 xiapi_sys::xiapi::XI_READBLK,
WRITEBLK 	=	 xiapi_sys::xiapi::XI_WRITEBLK,
NO_IMAGE 	=	 xiapi_sys::xiapi::XI_NO_IMAGE,
TIMEOUT 	=	 xiapi_sys::xiapi::XI_TIMEOUT,
INVALID_ARG 	=	 xiapi_sys::xiapi::XI_INVALID_ARG,
NOT_SUPPORTED 	=	 xiapi_sys::xiapi::XI_NOT_SUPPORTED,
ISOCH_ATTACH_BUFFERS 	=	 xiapi_sys::xiapi::XI_ISOCH_ATTACH_BUFFERS,
GET_OVERLAPPED_RESULT 	=	 xiapi_sys::xiapi::XI_GET_OVERLAPPED_RESULT,
MEMORY_ALLOCATION 	=	 xiapi_sys::xiapi::XI_MEMORY_ALLOCATION,
DLLCONTEXTISNULL 	=	 xiapi_sys::xiapi::XI_DLLCONTEXTISNULL,
DLLCONTEXTISNONZERO 	=	 xiapi_sys::xiapi::XI_DLLCONTEXTISNONZERO,
DLLCONTEXTEXIST 	=	 xiapi_sys::xiapi::XI_DLLCONTEXTEXIST,
TOOMANYDEVICES 	=	 xiapi_sys::xiapi::XI_TOOMANYDEVICES,
ERRORCAMCONTEXT 	=	 xiapi_sys::xiapi::XI_ERRORCAMCONTEXT,
UNKNOWN_HARDWARE 	=	 xiapi_sys::xiapi::XI_UNKNOWN_HARDWARE,
INVALID_TM_FILE 	=	 xiapi_sys::xiapi::XI_INVALID_TM_FILE,
INVALID_TM_TAG 	=	 xiapi_sys::xiapi::XI_INVALID_TM_TAG,
INCOMPLETE_TM 	=	 xiapi_sys::xiapi::XI_INCOMPLETE_TM,
BUS_RESET_FAILED 	=	 xiapi_sys::xiapi::XI_BUS_RESET_FAILED,
NOT_IMPLEMENTED 	=	 xiapi_sys::xiapi::XI_NOT_IMPLEMENTED,
SHADING_TOOBRIGHT 	=	 xiapi_sys::xiapi::XI_SHADING_TOOBRIGHT,
SHADING_TOODARK 	=	 xiapi_sys::xiapi::XI_SHADING_TOODARK,
TOO_LOW_GAIN 	=	 xiapi_sys::xiapi::XI_TOO_LOW_GAIN,
INVALID_BPL 	=	 xiapi_sys::xiapi::XI_INVALID_BPL,
BPL_REALLOC 	=	 xiapi_sys::xiapi::XI_BPL_REALLOC,
INVALID_PIXEL_LIST 	=	 xiapi_sys::xiapi::XI_INVALID_PIXEL_LIST,
INVALID_FFS 	=	 xiapi_sys::xiapi::XI_INVALID_FFS,
INVALID_PROFILE 	=	 xiapi_sys::xiapi::XI_INVALID_PROFILE,
INVALID_CALIBRATION 	=	 xiapi_sys::xiapi::XI_INVALID_CALIBRATION,
INVALID_BUFFER 	=	 xiapi_sys::xiapi::XI_INVALID_BUFFER,
INVALID_DATA 	=	 xiapi_sys::xiapi::XI_INVALID_DATA,
TGBUSY 	=	 xiapi_sys::xiapi::XI_TGBUSY,
IO_WRONG 	=	 xiapi_sys::xiapi::XI_IO_WRONG,
ACQUISITION_ALREADY_UP 	=	 xiapi_sys::xiapi::XI_ACQUISITION_ALREADY_UP,
OLD_DRIVER_VERSION 	=	 xiapi_sys::xiapi::XI_OLD_DRIVER_VERSION,
GET_LAST_ERROR 	=	 xiapi_sys::xiapi::XI_GET_LAST_ERROR,
CANT_PROCESS 	=	 xiapi_sys::xiapi::XI_CANT_PROCESS,
ACQUISITION_STOPED 	=	 xiapi_sys::xiapi::XI_ACQUISITION_STOPED,
ACQUISITION_STOPED_WERR 	=	 xiapi_sys::xiapi::XI_ACQUISITION_STOPED_WERR,
INVALID_INPUT_ICC_PROFILE 	=	 xiapi_sys::xiapi::XI_INVALID_INPUT_ICC_PROFILE,
INVALID_OUTPUT_ICC_PROFILE 	=	 xiapi_sys::xiapi::XI_INVALID_OUTPUT_ICC_PROFILE,
DEVICE_NOT_READY 	=	 xiapi_sys::xiapi::XI_DEVICE_NOT_READY,
SHADING_TOOCONTRAST 	=	 xiapi_sys::xiapi::XI_SHADING_TOOCONTRAST,
ALREADY_INITIALIZED 	=	 xiapi_sys::xiapi::XI_ALREADY_INITIALIZED,
NOT_ENOUGH_PRIVILEGES 	=	 xiapi_sys::xiapi::XI_NOT_ENOUGH_PRIVILEGES,
NOT_COMPATIBLE_DRIVER 	=	 xiapi_sys::xiapi::XI_NOT_COMPATIBLE_DRIVER,
TM_INVALID_RESOURCE 	=	 xiapi_sys::xiapi::XI_TM_INVALID_RESOURCE,
DEVICE_HAS_BEEN_RESETED 	=	 xiapi_sys::xiapi::XI_DEVICE_HAS_BEEN_RESETED,
NO_DEVICES_FOUND 	=	 xiapi_sys::xiapi::XI_NO_DEVICES_FOUND,
RESOURCE_OR_FUNCTION_LOCKED 	=	 xiapi_sys::xiapi::XI_RESOURCE_OR_FUNCTION_LOCKED,
BUFFER_SIZE_TOO_SMALL 	=	 xiapi_sys::xiapi::XI_BUFFER_SIZE_TOO_SMALL,
COULDNT_INIT_PROCESSOR 	=	 xiapi_sys::xiapi::XI_COULDNT_INIT_PROCESSOR,
NOT_INITIALIZED 	=	 xiapi_sys::xiapi::XI_NOT_INITIALIZED,
UNKNOWN_PARAM 	=	 xiapi_sys::xiapi::XI_UNKNOWN_PARAM,
WRONG_PARAM_VALUE 	=	 xiapi_sys::xiapi::XI_WRONG_PARAM_VALUE,
WRONG_PARAM_TYPE 	=	 xiapi_sys::xiapi::XI_WRONG_PARAM_TYPE,
WRONG_PARAM_SIZE 	=	 xiapi_sys::xiapi::XI_WRONG_PARAM_SIZE,
BUFFER_TOO_SMALL 	=	 xiapi_sys::xiapi::XI_BUFFER_TOO_SMALL,
NOT_SUPPORTED_PARAM 	=	 xiapi_sys::xiapi::XI_NOT_SUPPORTED_PARAM,
NOT_SUPPORTED_PARAM_INFO 	=	 xiapi_sys::xiapi::XI_NOT_SUPPORTED_PARAM_INFO,
NOT_SUPPORTED_DATA_FORMAT 	=	 xiapi_sys::xiapi::XI_NOT_SUPPORTED_DATA_FORMAT,
READ_ONLY_PARAM 	=	 xiapi_sys::xiapi::XI_READ_ONLY_PARAM,
BANDWIDTH_NOT_SUPPORTED 	=	 xiapi_sys::xiapi::XI_BANDWIDTH_NOT_SUPPORTED,
INVALID_FFS_FILE_NAME 	=	 xiapi_sys::xiapi::XI_INVALID_FFS_FILE_NAME,
FFS_FILE_NOT_FOUND 	=	 xiapi_sys::xiapi::XI_FFS_FILE_NOT_FOUND,
PROC_OTHER_ERROR 	=	 xiapi_sys::xiapi::XI_PROC_OTHER_ERROR,
PROC_PROCESSING_ERROR 	=	 xiapi_sys::xiapi::XI_PROC_PROCESSING_ERROR,
PROC_INPUT_FORMAT_UNSUPPORTED 	=	 xiapi_sys::xiapi::XI_PROC_INPUT_FORMAT_UNSUPPORTED,

}

pub enum DebugLevel {
    DETAIL 	=	 xiapi_sys::xiapi::XI_DL_DETAIL,
    TRACE 	=	 xiapi_sys::xiapi::XI_DL_TRACE,
    WARNING 	=	 xiapi_sys::xiapi::XI_DL_WARNING,
    ERROR 	=	 xiapi_sys::xiapi::XI_DL_ERROR,
    FATAL 	=	 xiapi_sys::xiapi::XI_DL_FATAL,
    DISABLED 	=	 xiapi_sys::xiapi::XI_DL_DISABLED,
}

pub enum ImgFormat {
    MONO8 	     =	 xiapi_sys::xiapi::XI_MONO8,
    MONO16       =	 xiapi_sys::xiapi::XI_MONO16,
    RGB24 	     =	 xiapi_sys::xiapi::XI_RGB24,
    RGB32 	     =	 xiapi_sys::xiapi::XI_RGB32,
    RGB_PLANAR 	 =	 xiapi_sys::xiapi::XI_RGB_PLANAR,
    RAW8 	     =	 xiapi_sys::xiapi::XI_RAW8,
    RAW16 	     =	 xiapi_sys::xiapi::XI_RAW16,
    FRM_TRANSPORT_DATA 	=	 xiapi_sys::xiapi::XI_FRM_TRANSPORT_DATA,
}

pub enum ColorFilterArray {
NONE 	=	 xiapi_sys::xiapi::XI_CFA_NONE,
BAYER_RGGB 	=	 xiapi_sys::xiapi::XI_CFA_BAYER_RGGB,
CMYG 	=	 xiapi_sys::xiapi::XI_CFA_CMYG,
RGR 	=	 xiapi_sys::xiapi::XI_CFA_RGR,
BAYER_BGGR 	=	 xiapi_sys::xiapi::XI_CFA_BAYER_BGGR,
BAYER_GRBG 	=	 xiapi_sys::xiapi::XI_CFA_BAYER_GRBG,
BAYER_GBRG 	=	 xiapi_sys::xiapi::XI_CFA_BAYER_GBRG,
}

pub enum BufferPolicy {
UNSAFE 	=	 xiapi_sys::xiapi::XI_BP_UNSAFE,
SAFE 	=	 xiapi_sys::xiapi::XI_BP_SAFE,
}

pub enum TriggerSource {
OFF 	=	 xiapi_sys::xiapi::XI_TRG_OFF,
EDGE_RISING 	=	 xiapi_sys::xiapi::XI_TRG_EDGE_RISING,
EDGE_FALLING 	=	 xiapi_sys::xiapi::XI_TRG_EDGE_FALLING,
SOFTWARE 	=	 xiapi_sys::xiapi::XI_TRG_SOFTWARE,
}

pub enum TriggerSelector {
FRAME_START 	=	 xiapi_sys::xiapi::XI_TRG_SEL_FRAME_START,
EXPOSURE_ACTIVE 	=	 xiapi_sys::xiapi::XI_TRG_SEL_EXPOSURE_ACTIVE,
FRAME_BURST_START 	=	 xiapi_sys::xiapi::XI_TRG_SEL_FRAME_BURST_START,
FRAME_BURST_ACTIVE 	=	 xiapi_sys::xiapi::XI_TRG_SEL_FRAME_BURST_ACTIVE,
MULTIPLE_EXPOSURES 	=	 xiapi_sys::xiapi::XI_TRG_SEL_MULTIPLE_EXPOSURES,
}

pub enum ACQTimingMode {
FREE_RUN 	=	 xiapi_sys::xiapi::XI_ACQ_TIMING_MODE_FREE_RUN,
FRAME_RATE 	=	 xiapi_sys::xiapi::XI_ACQ_TIMING_MODE_FRAME_RATE,
}

pub enum GPIMode {
OFF 	=	 xiapi_sys::xiapi::XI_GPI_OFF,
TRIGGER 	=	 xiapi_sys::xiapi::XI_GPI_TRIGGER,
EXT_EVENT 	=	 xiapi_sys::xiapi::XI_GPI_EXT_EVENT,
}

pub enum GPOMode {
OFF 	=	 xiapi_sys::xiapi::XI_GPO_OFF,
ON 	=	 xiapi_sys::xiapi::XI_GPO_ON,
FRAME_ACTIVE 	=	 xiapi_sys::xiapi::XI_GPO_FRAME_ACTIVE,
FRAME_ACTIVE_NEG 	=	 xiapi_sys::xiapi::XI_GPO_FRAME_ACTIVE_NEG,
EXPOSURE_ACTIVE 	=	 xiapi_sys::xiapi::XI_GPO_EXPOSURE_ACTIVE,
EXPOSURE_ACTIVE_NEG 	=	 xiapi_sys::xiapi::XI_GPO_EXPOSURE_ACTIVE_NEG,
FRAME_TRIGGER_WAIT 	=	 xiapi_sys::xiapi::XI_GPO_FRAME_TRIGGER_WAIT,
FRAME_TRIGGER_WAIT_NEG 	=	 xiapi_sys::xiapi::XI_GPO_FRAME_TRIGGER_WAIT_NEG,
EXPOSURE_PULSE 	=	 xiapi_sys::xiapi::XI_GPO_EXPOSURE_PULSE,
EXPOSURE_PULSE_NEG 	=	 xiapi_sys::xiapi::XI_GPO_EXPOSURE_PULSE_NEG,
BUSY 	=	 xiapi_sys::xiapi::XI_GPO_BUSY,
BUSY_NEG 	=	 xiapi_sys::xiapi::XI_GPO_BUSY_NEG,
}

pub enum LEDMode {
HEARTBEAT 	=	 xiapi_sys::xiapi::XI_LED_HEARTBEAT,
TRIGGER_ACTIVE 	=	 xiapi_sys::xiapi::XI_LED_TRIGGER_ACTIVE,
EXT_EVENT_ACTIVE 	=	 xiapi_sys::xiapi::XI_LED_EXT_EVENT_ACTIVE,
LINK 	=	 xiapi_sys::xiapi::XI_LED_LINK,
ACQUISITION 	=	 xiapi_sys::xiapi::XI_LED_ACQUISITION,
EXPOSURE_ACTIVE 	=	 xiapi_sys::xiapi::XI_LED_EXPOSURE_ACTIVE,
FRAME_ACTIVE 	=	 xiapi_sys::xiapi::XI_LED_FRAME_ACTIVE,
OFF 	=	 xiapi_sys::xiapi::XI_LED_OFF,
ON 	=	 xiapi_sys::xiapi::XI_LED_ON,
BLINK 	=	 xiapi_sys::xiapi::XI_LED_BLINK,
}

pub enum CounterSelector {
TRANSPORT_SKIPPED_FRAMES 	=	 xiapi_sys::xiapi::XI_CNT_SEL_TRANSPORT_SKIPPED_FRAMES,
API_SKIPPED_FRAMES 	=	 xiapi_sys::xiapi::XI_CNT_SEL_API_SKIPPED_FRAMES,
TRANSPORT_TRANSFERRED_FRAMES 	=	 xiapi_sys::xiapi::XI_CNT_SEL_TRANSPORT_TRANSFERRED_FRAMES,
}

pub enum TimestampResetMode {
ARM_ONCE 	=	 xiapi_sys::xiapi::XI_TS_RST_ARM_ONCE,
ARM_PERSIST 	=	 xiapi_sys::xiapi::XI_TS_RST_ARM_PERSIST,
}

pub enum TimestampResetSource {
OFF 	=	 xiapi_sys::xiapi::XI_TS_RST_OFF,
GPI_1 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_1,
GPI_2 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_2,
GPI_3 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_3,
GPI_4 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_4,
GPI_1_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_1_INV,
GPI_2_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_2_INV,
GPI_3_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_3_INV,
GPI_4_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPI_4_INV,
GPO_1 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_1,
GPO_2 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_2,
GPO_3 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_3,
GPO_4 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_4,
GPO_1_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_1_INV,
GPO_2_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_2_INV,
GPO_3_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_3_INV,
GPO_4_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_GPO_4_INV,
TRIGGER 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_TRIGGER,
TRIGGER_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_TRIGGER_INV,
SW 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_SW,
EXPACTIVE 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_EXPACTIVE,
EXPACTIVE_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_EXPACTIVE_INV,
FVAL 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_FVAL,
FVAL_INV 	=	 xiapi_sys::xiapi::XI_TS_RST_SRC_FVAL_INV,
}

pub enum PRMType {
Int     =   xiapi_sys::xiapi::xiTypeInteger,
Float   =   xiapi_sys::xiapi::xiTypeFloat,
Str     =   xiapi_sys::xiapi::xiTypeString,
}

pub enum Switch{
    OFF 	=	 xiapi_sys::xiapi::XI_OFF,
    ON 	=	 xiapi_sys::xiapi::XI_ON,
}

pub enum OutputDataPackignType{
DATA_PACK_XI_GROUPING 	=	 xiapi_sys::xiapi::XI_DATA_PACK_XI_GROUPING,
DATA_PACK_PFNC_LSB_PACKING 	=	 xiapi_sys::xiapi::XI_DATA_PACK_PFNC_LSB_PACKING,
}

pub enum DowndamplingType{
BINNING 	=	 xiapi_sys::xiapi::XI_BINNING,
SKIPPING 	=	 xiapi_sys::xiapi::XI_SKIPPING,
}

pub enum GainSelectorType{
GAIN_SELECTOR_ALL 	=	 xiapi_sys::xiapi::XI_GAIN_SELECTOR_ALL,
GAIN_SELECTOR_ANALOG_ALL 	=	 xiapi_sys::xiapi::XI_GAIN_SELECTOR_ANALOG_ALL,
GAIN_SELECTOR_DIGITAL_ALL 	=	 xiapi_sys::xiapi::XI_GAIN_SELECTOR_DIGITAL_ALL,
}

pub enum ShutterType{
SHUTTER_GLOBAL 	=	 xiapi_sys::xiapi::XI_SHUTTER_GLOBAL,
SHUTTER_ROLLING 	=	 xiapi_sys::xiapi::XI_SHUTTER_ROLLING,
SHUTTER_GLOBAL_RESET_RELEASE 	=	 xiapi_sys::xiapi::XI_SHUTTER_GLOBAL_RESET_RELEASE,
}

pub enum CMSMode{
CMS_DIS 	=	 xiapi_sys::xiapi::XI_CMS_DIS,
CMS_EN 	=	 xiapi_sys::xiapi::XI_CMS_EN,
CMS_EN_FAST 	=	 xiapi_sys::xiapi::XI_CMS_EN_FAST,
}

pub enum OpenBy{
OPEN_BY_INST_PATH 	=	 xiapi_sys::xiapi::XI_OPEN_BY_INST_PATH,
OPEN_BY_SN 	=	 xiapi_sys::xiapi::XI_OPEN_BY_SN,
OPEN_BY_USER_ID 	=	 xiapi_sys::xiapi::XI_OPEN_BY_USER_ID,
}

pub enum LensFeature{
LENS_FEATURE_MOTORIZED_FOCUS_SWITCH 	=	 xiapi_sys::xiapi::XI_LENS_FEATURE_MOTORIZED_FOCUS_SWITCH,
LENS_FEATURE_MOTORIZED_FOCUS_BOUNDED 	=	 xiapi_sys::xiapi::XI_LENS_FEATURE_MOTORIZED_FOCUS_BOUNDED,
LENS_FEATURE_MOTORIZED_FOCUS_CALIBRATION 	=	 xiapi_sys::xiapi::XI_LENS_FEATURE_MOTORIZED_FOCUS_CALIBRATION,
LENS_FEATURE_IMAGE_STABILIZATION_ENABLED 	=	 xiapi_sys::xiapi::XI_LENS_FEATURE_IMAGE_STABILIZATION_ENABLED,
LENS_FEATURE_IMAGE_STABILIZATION_SWITCH_STATUS  =   xiapi_sys::xiapi::XI_LENS_FEATURE_IMAGE_STABILIZATION_SWITCH_STATUS,
LENS_FEATURE_IMAGE_ZOOM_SUPPORTED 	=	 xiapi_sys::xiapi::XI_LENS_FEATURE_IMAGE_ZOOM_SUPPORTED,

}

pub enum SensorFeatureSelector{
SENSOR_FEATURE_ZEROROT_ENABLE 	=	 xiapi_sys::xiapi::XI_SENSOR_FEATURE_ZEROROT_ENABLE,

}

pub enum Parameter{
DEVICE_NAME, // Return device name
DEVICE_TYPE, // Return device type
DEVICE_MODEL_ID, // Return device model id
DEVICE_SN, // Return device serial number
DEVICE_SENS_SN, // Return sensor serial number
DEVICE_INSTANCE_PATH, // Return device system instance path.
DEVICE_USER_ID, // Return custom ID of camera.
// Device acquisition settings
EXPOSURE, // Exposure time in microseconds
GAIN_SELECTOR, // Gain selector for parameter Gain allows to select different type of gains. XI_GAIN_SELECTOR_TYPE
GAIN, // Gain in dB
DOWNSAMPLING, // Change image resolution by binning or skipping.
DOWNSAMPLING_TYPE, // Change image downsampling type. XI_DOWNSAMPLING_TYPE
SHUTTER_TYPE, // Change sensor shutter type(CMOS sensor). XI_SHUTTER_TYPE
IMAGE_DATA_FORMAT, // Output data format. XI_IMG_FORMAT
IMAGE_PAYLOAD_SIZE, // Buffer size in bytes sufficient for output image returned by xiGetImage
TRANSPORT_PIXEL_FORMAT, // Current format of pixels on transport layer. XI_GenTL_Image_Format_e
SENSOR_TAPS, // Number of taps
SENSOR_CLOCK_FREQ_HZ, // Sensor clock frequency in Hz.
SENSOR_CLOCK_FREQ_INDEX, // Sensor clock frequency index. Sensor with selected frequencies have possibility to set the frequency only by this index.
SENSOR_OUTPUT_CHANNEL_COUNT, // Number of output channels from sensor used for data transfer.
SENSOR_DATA_BIT_DEPTH, // Sensor output data bit depth.
OUTPUT_DATA_BIT_DEPTH, // Device output data bit depth.
IMAGE_DATA_BIT_DEPTH, // bitdepth of data returned by function xiGetImage
OUTPUT_DATA_PACKING, // Device output data packing (or grouping) enabled. Packing could be enabled if output_data_bit_depth > 8 and packing capability is available. XI_SWITCH
OUTPUT_DATA_PACKING_TYPE, // . XI_OUTPUT_DATA_PACKING_TYPE
FRAMERATE, // Define framerate in Hz
COUNTER_SELECTOR, // Select counter XI_COUNTER_SELECTOR
COUNTER_VALUE, // Counter status
ACQ_TIMING_MODE, // Type of sensor frames timing. XI_ACQ_TIMING_MODE
AVAILABLE_BANDWIDTH, // Calculate and return available interface bandwidth(int Megabits)
LIMIT_BANDWIDTH, // Set/get bandwidth(datarate)(in Megabits)
BUFFER_POLICY, // Data move policy XI_BP
WIDTH, // Width of the Image provided by the device (in pixels).
HEIGHT, // Height of the Image provided by the device (in pixels).
OFFSET_X, // Horizontal offset from the origin to the area of interest (in pixels).
OFFSET_Y, // Vertical offset from the origin to the area of interest (in pixels).
LUT_EN, // Activates LUT. XI_SWITCH
LUT_INDEX, // Control the index (offset) of the coefficient to access in the LUT.
LUT_VALUE, // Value at entry LUTIndex of the LUT
TRG_SOURCE, // Defines source of trigger. XI_TRG_SOURCE
TRG_SELECTOR, // Selects the type of trigger. XI_TRG_SELECTOR
TRG_SOFTWARE, // Generates an internal trigger. XI_PRM_TRG_SOURCE must be set to TRG_SOFTWARE.
TRG_DELAY, // Specifies the delay in microseconds (us) to apply after the trigger reception before activating it.
GPI_SELECTOR, // Selects GPI
GPI_MODE, // Defines GPI functionality XI_GPI_MODE
GPI_LEVEL, // GPI level
GPO_SELECTOR, // Selects GPO
GPO_MODE, // Defines GPO functionality XI_GPO_MODE
LED_SELECTOR, // Selects LED
LED_MODE, // Defines LED functionality XI_LED_MODE
TS_RST_MODE, // Defines how time stamp reset engine will be armed XI_TS_RST_MODE
TS_RST_SOURCE, // Defines which source will be used for timestamp reset. Writing this parameter will trigger settings of engine (arming) XI_TS_RST_SOURCE
ACQ_FRAME_BURST_COUNT, // Sets number of frames acquired by burst. This burst is used only if trigger is set to FrameBurstStart
// Extended Device parameters
IS_DEVICE_EXIST, // Returns 1 if camera connected and works properly. XI_SWITCH
ACQ_BUFFER_SIZE, // Acquisition buffer size in buffer_size_unit. Default bytes.
ACQ_BUFFER_SIZE_UNIT, // Acquisition buffer size unit in bytes. Default 1. E.g. Value 1024 means that buffer_size is in KiBytes
ACQ_TRANSPORT_BUFFER_SIZE, // Acquisition transport buffer size in bytes
BUFFERS_QUEUE_SIZE, // Queue of field/frame buffers
RECENT_FRAME, // GetImage returns most recent frame XI_SWITCH
// Color management settings
CMS, // Mode of color management system. XI_CMS_MODE
APPLY_CMS, // Enable applying of CMS profiles to xiGetImage (see XI_PRM_INPUT_CMS_PROFILE, XI_PRM_OUTPUT_CMS_PROFILE). XI_SWITCH
INPUT_CMS_PROFILE, // Filename for input cms profile (e.g. input.icc)
OUTPUT_CMS_PROFILE, // Filename for output cms profile (e.g. input.icc)
IMAGE_IS_COLOR, // Returns 1 for color cameras. XI_SWITCH
COLOR_FILTER_ARRAY, // Returns color filter array type of RAW data. XI_COLOR_FILTER_ARRAY
WB_KR, // White balance red coefficient
WB_KG, // White balance green coefficient
WB_KB, // White balance blue coefficient
MANUAL_WB, // Calculates White Balance(xiGetImage function must be called) XI_SWITCH
AUTO_WB, // Automatic white balance XI_SWITCH
GAMMAY, // Luminosity gamma
GAMMAC, // Chromaticity gamma
SHARPNESS, // Sharpness Strenght
CC_MATRIX_00, // Color Correction Matrix element [0][0]
CC_MATRIX_01, // Color Correction Matrix element [0][1]
CC_MATRIX_02, // Color Correction Matrix element [0][2]
CC_MATRIX_03, // Color Correction Matrix element [0][3]
CC_MATRIX_10, // Color Correction Matrix element [1][0]
CC_MATRIX_11, // Color Correction Matrix element [1][1]
CC_MATRIX_12, // Color Correction Matrix element [1][2]
CC_MATRIX_13, // Color Correction Matrix element [1][3]
CC_MATRIX_20, // Color Correction Matrix element [2][0]
CC_MATRIX_21, // Color Correction Matrix element [2][1]
CC_MATRIX_22, // Color Correction Matrix element [2][2]
CC_MATRIX_23, // Color Correction Matrix element [2][3]
CC_MATRIX_30, // Color Correction Matrix element [3][0]
CC_MATRIX_31, // Color Correction Matrix element [3][1]
CC_MATRIX_32, // Color Correction Matrix element [3][2]
CC_MATRIX_33, // Color Correction Matrix element [3][3]
DEFAULT_CC_MATRIX, // Set default Color Correction Matrix
// Automatic exposure/gain
AEAG, // Automatic exposure/gain XI_SWITCH
AEAG_ROI_OFFSET_X, // Automatic exposure/gain ROI offset X
AEAG_ROI_OFFSET_Y, // Automatic exposure/gain ROI offset Y
AEAG_ROI_WIDTH, // Automatic exposure/gain ROI Width
AEAG_ROI_HEIGHT, // Automatic exposure/gain ROI Height
EXP_PRIORITY, // Exposure priority (0.8 - exposure 80%, gain 20%).
AE_MAX_LIMIT, // Maximum time (us) used for exposure in AEAG procedure
AG_MAX_LIMIT, // Maximum limit of gain in AEAG procedure
AEAG_LEVEL, // Average intensity of output signal AEAG should achieve(in %)
// Bad Pixels Correction
BPC, // Correction of bad pixels XI_SWITCH
// Debounce
DEBOUNCE_EN, // Enable/Disable debounce to selected GPI XI_SWITCH
DEBOUNCE_T0, // Debounce time (x * 10us)
DEBOUNCE_T1, // Debounce time (x * 10us)
DEBOUNCE_POL, // Debounce polarity (pol = 1 t0 - falling edge, t1 - rising edge)
// Temperature control
IS_COOLED, // Returns 1 for cameras that support cooling.
COOLING, // Start camera cooling. XI_SWITCH
TARGET_TEMP, // Set sensor target temperature for cooling.
CHIP_TEMP, // Camera sensor temperature
HOUS_TEMP, // Camera housing tepmerature
// Sensor features
SENSOR_MODE, // Current sensor mode. Allows to select sensor mode by one integer. Setting of this parameter affects: image dimensions and downsampling.
HDR, // Enable High Dynamic Range feature. XI_SWITCH
HDR_KNEEPOINT_COUNT, // The number of kneepoints in the PWLR.
HDR_T1, // position of first kneepoint(in % of XI_PRM_EXPOSURE)
HDR_T2, // position of second kneepoint (in % of XI_PRM_EXPOSURE)
KNEEPOINT1, // value of first kneepoint (% of sensor saturation)
KNEEPOINT2, // value of second kneepoint (% of sensor saturation)
IMAGE_BLACK_LEVEL, // Last image black level counts. Can be used for Offline processing to recall it.
// Version info
API_VERSION, // Returns version of API.
DRV_VERSION, // Returns version of current device driver.
MCU1_VERSION, // Returns version of MCU1 firmware.
MCU2_VERSION, // Returns version of MCU2 firmware.
FPGA1_VERSION, // Returns version of FPGA1 firmware.
// API features
DEBUG_LEVEL, // Set debug level XI_DEBUG_LEVEL
AUTO_BANDWIDTH_CALCULATION, // Automatic bandwidth calculation, XI_SWITCH
// Camera FFS
READ_FILE_FFS, // Read file from camera flash filesystem.
WRITE_FILE_FFS, // Write file to camera flash filesystem.
FFS_FILE_NAME, // Set name of file to be written/read from camera FFS.
FREE_FFS_SIZE, // Size of free camera FFS.
USED_FFS_SIZE, // Size of used camera FFS.
FFS_ACCESS_KEY, // Setting of key enables file operations on some cameras.
// APIContextControl
API_CONTEXT_LIST, // List of current parameters settings context - parameters with values. Used for offline processing.
// Lens Control
LENS_MODE, // Status of lens control interface. This shall be set to XI_ON before any Lens operations. XI_SWITCH
LENS_APERTURE_VALUE, // Current lens aperture value in stops. Examples: 2.8, 4, 5.6, 8, 11
LENS_FOCUS_MOVEMENT_VALUE, // Lens current focus movement value to be used by XI_PRM_LENS_FOCUS_MOVE in motor steps.
LENS_FOCUS_MOVE, // Moves lens focus motor by steps set in XI_PRM_LENS_FOCUS_MOVEMENT_VALUE.
LENS_FOCUS_DISTANCE, // Lens focus distance in cm.
LENS_FOCAL_LENGTH, // Lens focal distance in mm.
LENS_FEATURE_SELECTOR, // Selects the current feature which is accessible by XI_PRM_LENS_FEATURE. XI_LENS_FEATURE
LENS_FEATURE, // Allows access to lens feature value currently selected by XI_PRM_LENS_FEATURE_SELECTOR.
// Sensor Control
SENSOR_FEATURE_SELECTOR, // Selects the current feature which is accessible by XI_PRM_SENSOR_FEATURE_VALUE. XI_SENSOR_FEATURE_SELECTOR
SENSOR_FEATURE_VALUE, // Allows access to sensor feature value currently selected by XI_PRM_SENSOR_FEATURE_SELECTOR.
}

//item, block, stmt, pat, expr, ty (type), ident, path, tt
macro_rules! parameter_to_string {
    ( $s:expr, $( $x:pat, $y:expr );* ) => {
        {
            match ($s) {
                $(
                    $x =>{
                        return CString::new($y).unwrap().as_ptr();
                    }
                )*
                _ => {
                    return CString::new(xiapi_sys::xiapi::XI_PRM_DEVICE_TYPE).unwrap().as_ptr();
                }
            }
        }
    };
}

pub impl Into<*const c_char> for Parameter{
    pub fn into(&self) -> *const c_char {
        parameter_to_string!(self,
            Parameter::DEVICE_NAME,  xiapi_sys::xiapi::XI_PRM_DEVICE_NAME;
            Parameter::DEVICE_TYPE,  xiapi_sys::xiapi::XI_PRM_DEVICE_TYPE;
            Parameter::DEVICE_MODEL_ID,  xiapi_sys::xiapi::XI_PRM_DEVICE_MODEL_ID;
            Parameter::DEVICE_SN,  xiapi_sys::xiapi::XI_PRM_DEVICE_SN;
            Parameter::DEVICE_SENS_SN,  xiapi_sys::xiapi::XI_PRM_DEVICE_SENS_SN;
            Parameter::DEVICE_INSTANCE_PATH,  xiapi_sys::xiapi::XI_PRM_DEVICE_INSTANCE_PATH;
            Parameter::DEVICE_USER_ID,  xiapi_sys::xiapi::XI_PRM_DEVICE_USER_ID;
            // Device acquisition settings
            Parameter::EXPOSURE,  xiapi_sys::xiapi::XI_PRM_EXPOSURE;
            Parameter::GAIN_SELECTOR,  xiapi_sys::xiapi::XI_PRM_GAIN_SELECTOR;
            Parameter::GAIN,  xiapi_sys::xiapi::XI_PRM_GAIN;
            Parameter::DOWNSAMPLING,  xiapi_sys::xiapi::XI_PRM_DOWNSAMPLING;
            Parameter::DOWNSAMPLING_TYPE,  xiapi_sys::xiapi::XI_PRM_DOWNSAMPLING_TYPE;
            Parameter::SHUTTER_TYPE,  xiapi_sys::xiapi::XI_PRM_SHUTTER_TYPE;
            Parameter::IMAGE_DATA_FORMAT,  xiapi_sys::xiapi::XI_PRM_IMAGE_DATA_FORMAT;
            Parameter::IMAGE_PAYLOAD_SIZE,  xiapi_sys::xiapi::XI_PRM_IMAGE_PAYLOAD_SIZE;
            Parameter::TRANSPORT_PIXEL_FORMAT,  xiapi_sys::xiapi::XI_PRM_TRANSPORT_PIXEL_FORMAT;
            Parameter::SENSOR_TAPS,  xiapi_sys::xiapi::XI_PRM_SENSOR_TAPS;
            Parameter::SENSOR_CLOCK_FREQ_HZ,  xiapi_sys::xiapi::XI_PRM_SENSOR_CLOCK_FREQ_HZ;
            Parameter::SENSOR_CLOCK_FREQ_INDEX,  xiapi_sys::xiapi::XI_PRM_SENSOR_CLOCK_FREQ_INDEX;
            Parameter::SENSOR_OUTPUT_CHANNEL_COUNT,  xiapi_sys::xiapi::XI_PRM_SENSOR_OUTPUT_CHANNEL_COUNT;
            Parameter::SENSOR_DATA_BIT_DEPTH,  xiapi_sys::xiapi::XI_PRM_SENSOR_DATA_BIT_DEPTH;
            Parameter::OUTPUT_DATA_BIT_DEPTH,  xiapi_sys::xiapi::XI_PRM_OUTPUT_DATA_BIT_DEPTH;
            Parameter::IMAGE_DATA_BIT_DEPTH,  xiapi_sys::xiapi::XI_PRM_IMAGE_DATA_BIT_DEPTH;
            Parameter::OUTPUT_DATA_PACKING,  xiapi_sys::xiapi::XI_PRM_OUTPUT_DATA_PACKING;
            Parameter::OUTPUT_DATA_PACKING_TYPE,  xiapi_sys::xiapi::XI_PRM_OUTPUT_DATA_PACKING_TYPE;
            Parameter::FRAMERATE,  xiapi_sys::xiapi::XI_PRM_FRAMERATE;
            Parameter::COUNTER_SELECTOR,  xiapi_sys::xiapi::XI_PRM_COUNTER_SELECTOR;
            Parameter::COUNTER_VALUE,  xiapi_sys::xiapi::XI_PRM_COUNTER_VALUE;
            Parameter::ACQ_TIMING_MODE,  xiapi_sys::xiapi::XI_PRM_ACQ_TIMING_MODE;
            Parameter::AVAILABLE_BANDWIDTH,  xiapi_sys::xiapi::XI_PRM_AVAILABLE_BANDWIDTH;
            Parameter::LIMIT_BANDWIDTH,  xiapi_sys::xiapi::XI_PRM_LIMIT_BANDWIDTH;
            Parameter::BUFFER_POLICY,  xiapi_sys::xiapi::XI_PRM_BUFFER_POLICY;
            Parameter::WIDTH,  xiapi_sys::xiapi::XI_PRM_WIDTH;
            Parameter::HEIGHT,  xiapi_sys::xiapi::XI_PRM_HEIGHT;
            Parameter::OFFSET_X,  xiapi_sys::xiapi::XI_PRM_OFFSET_X;
            Parameter::OFFSET_Y,  xiapi_sys::xiapi::XI_PRM_OFFSET_Y;
            Parameter::LUT_EN,  xiapi_sys::xiapi::XI_PRM_LUT_EN;
            Parameter::LUT_INDEX,  xiapi_sys::xiapi::XI_PRM_LUT_INDEX;
            Parameter::LUT_VALUE,  xiapi_sys::xiapi::XI_PRM_LUT_VALUE;
            Parameter::TRG_SOURCE,  xiapi_sys::xiapi::XI_PRM_TRG_SOURCE;
            Parameter::TRG_SELECTOR,  xiapi_sys::xiapi::XI_PRM_TRG_SELECTOR;
            Parameter::TRG_SOFTWARE,  xiapi_sys::xiapi::XI_PRM_TRG_SOFTWARE;
            Parameter::TRG_DELAY,  xiapi_sys::xiapi::XI_PRM_TRG_DELAY;
            Parameter::GPI_SELECTOR,  xiapi_sys::xiapi::XI_PRM_GPI_SELECTOR;
            Parameter::GPI_MODE,  xiapi_sys::xiapi::XI_PRM_GPI_MODE;
            Parameter::GPI_LEVEL,  xiapi_sys::xiapi::XI_PRM_GPI_LEVEL;
            Parameter::GPO_SELECTOR,  xiapi_sys::xiapi::XI_PRM_GPO_SELECTOR;
            Parameter::GPO_MODE,  xiapi_sys::xiapi::XI_PRM_GPO_MODE;
            Parameter::LED_SELECTOR,  xiapi_sys::xiapi::XI_PRM_LED_SELECTOR;
            Parameter::LED_MODE,  xiapi_sys::xiapi::XI_PRM_LED_MODE;
            Parameter::TS_RST_MODE,  xiapi_sys::xiapi::XI_PRM_TS_RST_MODE;
            Parameter::TS_RST_SOURCE,  xiapi_sys::xiapi::XI_PRM_TS_RST_SOURCE;
            Parameter::ACQ_FRAME_BURST_COUNT,  xiapi_sys::xiapi::XI_PRM_ACQ_FRAME_BURST_COUNT;
            // Extended Device parameters
            Parameter::IS_DEVICE_EXIST,  xiapi_sys::xiapi::XI_PRM_IS_DEVICE_EXIST;
            Parameter::ACQ_BUFFER_SIZE,  xiapi_sys::xiapi::XI_PRM_ACQ_BUFFER_SIZE;
            Parameter::ACQ_BUFFER_SIZE_UNIT,  xiapi_sys::xiapi::XI_PRM_ACQ_BUFFER_SIZE_UNIT;
            Parameter::ACQ_TRANSPORT_BUFFER_SIZE,  xiapi_sys::xiapi::XI_PRM_ACQ_TRANSPORT_BUFFER_SIZE;
            Parameter::BUFFERS_QUEUE_SIZE,  xiapi_sys::xiapi::XI_PRM_BUFFERS_QUEUE_SIZE;
            Parameter::RECENT_FRAME,  xiapi_sys::xiapi::XI_PRM_RECENT_FRAME;
            // Color management settings
            Parameter::CMS,  xiapi_sys::xiapi::XI_PRM_CMS;
            Parameter::APPLY_CMS,  xiapi_sys::xiapi::XI_PRM_APPLY_CMS;
            Parameter::INPUT_CMS_PROFILE,  xiapi_sys::xiapi::XI_PRM_INPUT_CMS_PROFILE;
            Parameter::OUTPUT_CMS_PROFILE,  xiapi_sys::xiapi::XI_PRM_OUTPUT_CMS_PROFILE;
            Parameter::IMAGE_IS_COLOR,  xiapi_sys::xiapi::XI_PRM_IMAGE_IS_COLOR;
            Parameter::COLOR_FILTER_ARRAY,  xiapi_sys::xiapi::XI_PRM_COLOR_FILTER_ARRAY;
            Parameter::WB_KR,  xiapi_sys::xiapi::XI_PRM_WB_KR;
            Parameter::WB_KG,  xiapi_sys::xiapi::XI_PRM_WB_KG;
            Parameter::WB_KB,  xiapi_sys::xiapi::XI_PRM_WB_KB;
            Parameter::MANUAL_WB,  xiapi_sys::xiapi::XI_PRM_MANUAL_WB;
            Parameter::AUTO_WB,  xiapi_sys::xiapi::XI_PRM_AUTO_WB;
            Parameter::GAMMAY,  xiapi_sys::xiapi::XI_PRM_GAMMAY;
            Parameter::GAMMAC,  xiapi_sys::xiapi::XI_PRM_GAMMAC;
            Parameter::SHARPNESS,  xiapi_sys::xiapi::XI_PRM_SHARPNESS;
            Parameter::CC_MATRIX_00,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_00;
            Parameter::CC_MATRIX_01,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_01;
            Parameter::CC_MATRIX_02,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_02;
            Parameter::CC_MATRIX_03,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_03;
            Parameter::CC_MATRIX_10,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_10;
            Parameter::CC_MATRIX_11,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_11;
            Parameter::CC_MATRIX_12,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_12;
            Parameter::CC_MATRIX_13,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_13;
            Parameter::CC_MATRIX_20,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_20;
            Parameter::CC_MATRIX_21,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_21;
            Parameter::CC_MATRIX_22,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_22;
            Parameter::CC_MATRIX_23,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_23;
            Parameter::CC_MATRIX_30,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_30;
            Parameter::CC_MATRIX_31,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_31;
            Parameter::CC_MATRIX_32,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_32;
            Parameter::CC_MATRIX_33,  xiapi_sys::xiapi::XI_PRM_CC_MATRIX_33;
            Parameter::DEFAULT_CC_MATRIX,  xiapi_sys::xiapi::XI_PRM_DEFAULT_CC_MATRIX;
            // Automatic exposure/gain
            Parameter::AEAG,  xiapi_sys::xiapi::XI_PRM_AEAG;
            Parameter::AEAG_ROI_OFFSET_X,  xiapi_sys::xiapi::XI_PRM_AEAG_ROI_OFFSET_X;
            Parameter::AEAG_ROI_OFFSET_Y,  xiapi_sys::xiapi::XI_PRM_AEAG_ROI_OFFSET_Y;
            Parameter::AEAG_ROI_WIDTH,  xiapi_sys::xiapi::XI_PRM_AEAG_ROI_WIDTH;
            Parameter::AEAG_ROI_HEIGHT,  xiapi_sys::xiapi::XI_PRM_AEAG_ROI_HEIGHT;
            Parameter::EXP_PRIORITY,  xiapi_sys::xiapi::XI_PRM_EXP_PRIORITY;
            Parameter::AE_MAX_LIMIT,  xiapi_sys::xiapi::XI_PRM_AE_MAX_LIMIT;
            Parameter::AG_MAX_LIMIT,  xiapi_sys::xiapi::XI_PRM_AG_MAX_LIMIT;
            Parameter::AEAG_LEVEL,  xiapi_sys::xiapi::XI_PRM_AEAG_LEVEL;
            // Bad Pixels Correction
            Parameter::BPC,  xiapi_sys::xiapi::XI_PRM_BPC;
            // Debounce
            Parameter::DEBOUNCE_EN,  xiapi_sys::xiapi::XI_PRM_DEBOUNCE_EN;
            Parameter::DEBOUNCE_T0,  xiapi_sys::xiapi::XI_PRM_DEBOUNCE_T0;
            Parameter::DEBOUNCE_T1,  xiapi_sys::xiapi::XI_PRM_DEBOUNCE_T1;
            Parameter::DEBOUNCE_POL,  xiapi_sys::xiapi::XI_PRM_DEBOUNCE_POL;
            // Temperature control
            Parameter::IS_COOLED,  xiapi_sys::xiapi::XI_PRM_IS_COOLED;
            Parameter::COOLING,  xiapi_sys::xiapi::XI_PRM_COOLING;
            Parameter::TARGET_TEMP,  xiapi_sys::xiapi::XI_PRM_TARGET_TEMP;
            Parameter::CHIP_TEMP,  xiapi_sys::xiapi::XI_PRM_CHIP_TEMP;
            Parameter::HOUS_TEMP,  xiapi_sys::xiapi::XI_PRM_HOUS_TEMP;
            // Sensor features
            Parameter::SENSOR_MODE,  xiapi_sys::xiapi::XI_PRM_SENSOR_MODE;
            Parameter::HDR,  xiapi_sys::xiapi::XI_PRM_HDR;
            Parameter::HDR_KNEEPOINT_COUNT,  xiapi_sys::xiapi::XI_PRM_HDR_KNEEPOINT_COUNT;
            Parameter::HDR_T1,  xiapi_sys::xiapi::XI_PRM_HDR_T1;
            Parameter::HDR_T2,  xiapi_sys::xiapi::XI_PRM_HDR_T2;
            Parameter::KNEEPOINT1,  xiapi_sys::xiapi::XI_PRM_KNEEPOINT1;
            Parameter::KNEEPOINT2,  xiapi_sys::xiapi::XI_PRM_KNEEPOINT2;
            Parameter::IMAGE_BLACK_LEVEL,  xiapi_sys::xiapi::XI_PRM_IMAGE_BLACK_LEVEL;
            // Version info
            Parameter::API_VERSION,  xiapi_sys::xiapi::XI_PRM_API_VERSION;
            Parameter::DRV_VERSION,  xiapi_sys::xiapi::XI_PRM_DRV_VERSION;
            Parameter::MCU1_VERSION,  xiapi_sys::xiapi::XI_PRM_MCU1_VERSION;
            Parameter::MCU2_VERSION,  xiapi_sys::xiapi::XI_PRM_MCU2_VERSION;
            Parameter::FPGA1_VERSION,  xiapi_sys::xiapi::XI_PRM_FPGA1_VERSION;
            // API features
            Parameter::DEBUG_LEVEL,  xiapi_sys::xiapi::XI_PRM_DEBUG_LEVEL;
            Parameter::AUTO_BANDWIDTH_CALCULATION,  xiapi_sys::xiapi::XI_PRM_AUTO_BANDWIDTH_CALCULATION;
            // Camera FFS
            Parameter::READ_FILE_FFS,  xiapi_sys::xiapi::XI_PRM_READ_FILE_FFS;
            Parameter::WRITE_FILE_FFS,  xiapi_sys::xiapi::XI_PRM_WRITE_FILE_FFS;
            Parameter::FFS_FILE_NAME,  xiapi_sys::xiapi::XI_PRM_FFS_FILE_NAME;
            Parameter::FREE_FFS_SIZE,  xiapi_sys::xiapi::XI_PRM_FREE_FFS_SIZE;
            Parameter::USED_FFS_SIZE,  xiapi_sys::xiapi::XI_PRM_USED_FFS_SIZE;
            Parameter::FFS_ACCESS_KEY,  xiapi_sys::xiapi::XI_PRM_FFS_ACCESS_KEY;
            // APIContextControl
            Parameter::API_CONTEXT_LIST,  xiapi_sys::xiapi::XI_PRM_API_CONTEXT_LIST;
            // Lens Control
            Parameter::LENS_MODE,  xiapi_sys::xiapi::XI_PRM_LENS_MODE;
            Parameter::LENS_APERTURE_VALUE,  xiapi_sys::xiapi::XI_PRM_LENS_APERTURE_VALUE;
            Parameter::LENS_FOCUS_MOVEMENT_VALUE,  xiapi_sys::xiapi::XI_PRM_LENS_FOCUS_MOVEMENT_VALUE;
            Parameter::LENS_FOCUS_MOVE,  xiapi_sys::xiapi::XI_PRM_LENS_FOCUS_MOVE;
            Parameter::LENS_FOCUS_DISTANCE,  xiapi_sys::xiapi::XI_PRM_LENS_FOCUS_DISTANCE;
            Parameter::LENS_FOCAL_LENGTH,  xiapi_sys::xiapi::XI_PRM_LENS_FOCAL_LENGTH;
            Parameter::LENS_FEATURE_SELECTOR,  xiapi_sys::xiapi::XI_PRM_LENS_FEATURE_SELECTOR;
            Parameter::LENS_FEATURE,  xiapi_sys::xiapi::XI_PRM_LENS_FEATURE;
            // Sensor Control
            Parameter::SENSOR_FEATURE_SELECTOR,  xiapi_sys::xiapi::XI_PRM_SENSOR_FEATURE_SELECTOR;
            Parameter::SENSOR_FEATURE_VALUE, xiapi_sys::xiapi::XI_PRM_SENSOR_FEATURE_VALUE
            )
        }
    }

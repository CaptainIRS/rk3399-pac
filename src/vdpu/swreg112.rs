#[doc = "Register `SWREG112` reader"]
pub type R = crate::R<Swreg112Spec>;
#[doc = "Register `SWREG112` writer"]
pub type W = crate::W<Swreg112Spec>;
#[doc = "Field `H264_CURFRM_NUM` reader - the current frame number for h264\n\nit may be use for reference picture reordering and identify\n\nshort-term reference frames"]
pub type H264CurfrmNumR = crate::FieldReader<u16>;
#[doc = "Field `H264_CURFRM_NUM` writer - the current frame number for h264\n\nit may be use for reference picture reordering and identify\n\nshort-term reference frames"]
pub type H264CurfrmNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_CURFRM_LEN` reader - the bit length of input data stream's frame num\n\nH.264: Bit length of frame_num in data stream"]
pub type H264CurfrmLenR = crate::FieldReader;
#[doc = "Field `H264_CURFRM_LEN` writer - the bit length of input data stream's frame num\n\nH.264: Bit length of frame_num in data stream"]
pub type H264CurfrmLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_RPCP_FLAG` reader - redundant picture count present flag\n\nto specifies whether redundant picture count syntax elements"]
pub type H264RpcpFlagR = crate::BitReader;
#[doc = "Field `H264_RPCP_FLAG` writer - redundant picture count present flag\n\nto specifies whether redundant picture count syntax elements"]
pub type H264RpcpFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_DBLK_CTRL_FLAG` reader - the control present flag of deblocking filter\n\nto indicates if the slice header will have the deblocking filter's extra\n\nvariables controlling characteristics"]
pub type H264DblkCtrlFlagR = crate::BitReader;
#[doc = "Field `H264_DBLK_CTRL_FLAG` writer - the control present flag of deblocking filter\n\nto indicates if the slice header will have the deblocking filter's extra\n\nvariables controlling characteristics"]
pub type H264DblkCtrlFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - the current frame number for h264\n\nit may be use for reference picture reordering and identify\n\nshort-term reference frames"]
    #[inline(always)]
    pub fn h264_curfrm_num(&self) -> H264CurfrmNumR {
        H264CurfrmNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - the bit length of input data stream's frame num\n\nH.264: Bit length of frame_num in data stream"]
    #[inline(always)]
    pub fn h264_curfrm_len(&self) -> H264CurfrmLenR {
        H264CurfrmLenR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - redundant picture count present flag\n\nto specifies whether redundant picture count syntax elements"]
    #[inline(always)]
    pub fn h264_rpcp_flag(&self) -> H264RpcpFlagR {
        H264RpcpFlagR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - the control present flag of deblocking filter\n\nto indicates if the slice header will have the deblocking filter's extra\n\nvariables controlling characteristics"]
    #[inline(always)]
    pub fn h264_dblk_ctrl_flag(&self) -> H264DblkCtrlFlagR {
        H264DblkCtrlFlagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the current frame number for h264\n\nit may be use for reference picture reordering and identify\n\nshort-term reference frames"]
    #[inline(always)]
    #[must_use]
    pub fn h264_curfrm_num(&mut self) -> H264CurfrmNumW<Swreg112Spec> {
        H264CurfrmNumW::new(self, 0)
    }
    #[doc = "Bits 16:20 - the bit length of input data stream's frame num\n\nH.264: Bit length of frame_num in data stream"]
    #[inline(always)]
    #[must_use]
    pub fn h264_curfrm_len(&mut self) -> H264CurfrmLenW<Swreg112Spec> {
        H264CurfrmLenW::new(self, 16)
    }
    #[doc = "Bit 30 - redundant picture count present flag\n\nto specifies whether redundant picture count syntax elements"]
    #[inline(always)]
    #[must_use]
    pub fn h264_rpcp_flag(&mut self) -> H264RpcpFlagW<Swreg112Spec> {
        H264RpcpFlagW::new(self, 30)
    }
    #[doc = "Bit 31 - the control present flag of deblocking filter\n\nto indicates if the slice header will have the deblocking filter's extra\n\nvariables controlling characteristics"]
    #[inline(always)]
    #[must_use]
    pub fn h264_dblk_ctrl_flag(&mut self) -> H264DblkCtrlFlagW<Swreg112Spec> {
        H264DblkCtrlFlagW::new(self, 31)
    }
}
#[doc = "current frame related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg112::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg112::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg112Spec;
impl crate::RegisterSpec for Swreg112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg112::R`](R) reader structure"]
impl crate::Readable for Swreg112Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg112::W`](W) writer structure"]
impl crate::Writable for Swreg112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG112 to value 0"]
impl crate::Resettable for Swreg112Spec {
    const RESET_VALUE: u32 = 0;
}

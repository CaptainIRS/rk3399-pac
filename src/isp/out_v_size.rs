#[doc = "Register `OUT_V_SIZE` reader"]
pub type R = crate::R<OutVSizeSpec>;
#[doc = "Register `OUT_V_SIZE` writer"]
pub type W = crate::W<OutVSizeSpec>;
#[doc = "Field `ISP_OUT_V_SIZE` reader - vertical pic size in lines"]
pub type IspOutVSizeR = crate::FieldReader<u16>;
#[doc = "Field `ISP_OUT_V_SIZE` writer - vertical pic size in lines"]
pub type IspOutVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical pic size in lines"]
    #[inline(always)]
    pub fn isp_out_v_size(&self) -> IspOutVSizeR {
        IspOutVSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical pic size in lines"]
    #[inline(always)]
    #[must_use]
    pub fn isp_out_v_size(&mut self) -> IspOutVSizeW<OutVSizeSpec> {
        IspOutVSizeW::new(self, 0)
    }
}
#[doc = "Output vertical picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutVSizeSpec;
impl crate::RegisterSpec for OutVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_v_size::R`](R) reader structure"]
impl crate::Readable for OutVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`out_v_size::W`](W) writer structure"]
impl crate::Writable for OutVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_V_SIZE to value 0x0c00"]
impl crate::Resettable for OutVSizeSpec {
    const RESET_VALUE: u32 = 0x0c00;
}

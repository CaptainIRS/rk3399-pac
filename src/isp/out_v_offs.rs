#[doc = "Register `OUT_V_OFFS` reader"]
pub type R = crate::R<OutVOffsSpec>;
#[doc = "Register `OUT_V_OFFS` writer"]
pub type W = crate::W<OutVOffsSpec>;
#[doc = "Field `ISP_OUT_V_OFFS` reader - vertical pic offset in lines\n\n"]
pub type IspOutVOffsR = crate::FieldReader<u16>;
#[doc = "Field `ISP_OUT_V_OFFS` writer - vertical pic offset in lines\n\n"]
pub type IspOutVOffsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical pic offset in lines\n\n"]
    #[inline(always)]
    pub fn isp_out_v_offs(&self) -> IspOutVOffsR {
        IspOutVOffsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical pic offset in lines\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn isp_out_v_offs(&mut self) -> IspOutVOffsW<OutVOffsSpec> {
        IspOutVOffsW::new(self, 0)
    }
}
#[doc = "Vertical offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_v_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_v_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutVOffsSpec;
impl crate::RegisterSpec for OutVOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_v_offs::R`](R) reader structure"]
impl crate::Readable for OutVOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`out_v_offs::W`](W) writer structure"]
impl crate::Writable for OutVOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_V_OFFS to value 0"]
impl crate::Resettable for OutVOffsSpec {
    const RESET_VALUE: u32 = 0;
}

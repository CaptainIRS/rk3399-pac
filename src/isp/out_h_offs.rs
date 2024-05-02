#[doc = "Register `OUT_H_OFFS` reader"]
pub type R = crate::R<OutHOffsSpec>;
#[doc = "Register `OUT_H_OFFS` writer"]
pub type W = crate::W<OutHOffsSpec>;
#[doc = "Field `ISP_OUT_H_OFFS` reader - vertical pic offset in lines"]
pub type IspOutHOffsR = crate::FieldReader<u16>;
#[doc = "Field `ISP_OUT_H_OFFS` writer - vertical pic offset in lines"]
pub type IspOutHOffsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical pic offset in lines"]
    #[inline(always)]
    pub fn isp_out_h_offs(&self) -> IspOutHOffsR {
        IspOutHOffsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical pic offset in lines"]
    #[inline(always)]
    #[must_use]
    pub fn isp_out_h_offs(&mut self) -> IspOutHOffsW<OutHOffsSpec> {
        IspOutHOffsW::new(self, 0)
    }
}
#[doc = "Horizontal offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_h_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_h_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutHOffsSpec;
impl crate::RegisterSpec for OutHOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_h_offs::R`](R) reader structure"]
impl crate::Readable for OutHOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`out_h_offs::W`](W) writer structure"]
impl crate::Writable for OutHOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_H_OFFS to value 0"]
impl crate::Resettable for OutHOffsSpec {
    const RESET_VALUE: u32 = 0;
}

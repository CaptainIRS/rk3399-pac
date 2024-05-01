#[doc = "Register `DST_BASE1` reader"]
pub type R = crate::R<DstBase1Spec>;
#[doc = "Register `DST_BASE1` writer"]
pub type W = crate::W<DstBase1Spec>;
#[doc = "Field `SW_DST_BASE1` reader - destination image Cb/CbCr base address"]
pub type SwDstBase1R = crate::FieldReader<u32>;
#[doc = "Field `SW_DST_BASE1` writer - destination image Cb/CbCr base address"]
pub type SwDstBase1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - destination image Cb/CbCr base address"]
    #[inline(always)]
    pub fn sw_dst_base1(&self) -> SwDstBase1R {
        SwDstBase1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - destination image Cb/CbCr base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_base1(&mut self) -> SwDstBase1W<DstBase1Spec> {
        SwDstBase1W::new(self, 0)
    }
}
#[doc = "RGA destination image base address 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_base1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_base1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstBase1Spec;
impl crate::RegisterSpec for DstBase1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_base1::R`](R) reader structure"]
impl crate::Readable for DstBase1Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_base1::W`](W) writer structure"]
impl crate::Writable for DstBase1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_BASE1 to value 0"]
impl crate::Resettable for DstBase1Spec {
    const RESET_VALUE: u32 = 0;
}

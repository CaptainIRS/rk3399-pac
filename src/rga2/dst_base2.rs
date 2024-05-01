#[doc = "Register `DST_BASE2` reader"]
pub type R = crate::R<DstBase2Spec>;
#[doc = "Register `DST_BASE2` writer"]
pub type W = crate::W<DstBase2Spec>;
#[doc = "Field `SW_DST_BASE2` reader - destination image Cr base address"]
pub type SwDstBase2R = crate::FieldReader<u32>;
#[doc = "Field `SW_DST_BASE2` writer - destination image Cr base address"]
pub type SwDstBase2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - destination image Cr base address"]
    #[inline(always)]
    pub fn sw_dst_base2(&self) -> SwDstBase2R {
        SwDstBase2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - destination image Cr base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_base2(&mut self) -> SwDstBase2W<DstBase2Spec> {
        SwDstBase2W::new(self, 0)
    }
}
#[doc = "RGA destination image base address 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_base2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_base2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstBase2Spec;
impl crate::RegisterSpec for DstBase2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_base2::R`](R) reader structure"]
impl crate::Readable for DstBase2Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_base2::W`](W) writer structure"]
impl crate::Writable for DstBase2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_BASE2 to value 0"]
impl crate::Resettable for DstBase2Spec {
    const RESET_VALUE: u32 = 0;
}

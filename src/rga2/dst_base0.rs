#[doc = "Register `DST_BASE0` reader"]
pub type R = crate::R<DstBase0Spec>;
#[doc = "Register `DST_BASE0` writer"]
pub type W = crate::W<DstBase0Spec>;
#[doc = "Field `SW_DST_BASE0` reader - destination image Y/RGB base address"]
pub type SwDstBase0R = crate::FieldReader<u32>;
#[doc = "Field `SW_DST_BASE0` writer - destination image Y/RGB base address"]
pub type SwDstBase0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - destination image Y/RGB base address"]
    #[inline(always)]
    pub fn sw_dst_base0(&self) -> SwDstBase0R {
        SwDstBase0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - destination image Y/RGB base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_base0(&mut self) -> SwDstBase0W<DstBase0Spec> {
        SwDstBase0W::new(self, 0)
    }
}
#[doc = "RGA destination image base address 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_base0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_base0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstBase0Spec;
impl crate::RegisterSpec for DstBase0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_base0::R`](R) reader structure"]
impl crate::Readable for DstBase0Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_base0::W`](W) writer structure"]
impl crate::Writable for DstBase0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_BASE0 to value 0"]
impl crate::Resettable for DstBase0Spec {
    const RESET_VALUE: u32 = 0;
}

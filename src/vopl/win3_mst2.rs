#[doc = "Register `WIN3_MST2` reader"]
pub type R = crate::R<Win3Mst2Spec>;
#[doc = "Register `WIN3_MST2` writer"]
pub type W = crate::W<Win3Mst2Spec>;
#[doc = "Field `WIN3_MST2` reader - Win3 frame buffer memory start address2\n\n*must be alianed to 8byte address"]
pub type Win3Mst2R = crate::FieldReader<u32>;
#[doc = "Field `WIN3_MST2` writer - Win3 frame buffer memory start address2\n\n*must be alianed to 8byte address"]
pub type Win3Mst2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Win3 frame buffer memory start address2\n\n*must be alianed to 8byte address"]
    #[inline(always)]
    pub fn win3_mst2(&self) -> Win3Mst2R {
        Win3Mst2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Win3 frame buffer memory start address2\n\n*must be alianed to 8byte address"]
    #[inline(always)]
    #[must_use]
    pub fn win3_mst2(&mut self) -> Win3Mst2W<Win3Mst2Spec> {
        Win3Mst2W::new(self, 0)
    }
}
#[doc = "Win3 memory start address2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_mst2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_mst2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Mst2Spec;
impl crate::RegisterSpec for Win3Mst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_mst2::R`](R) reader structure"]
impl crate::Readable for Win3Mst2Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_mst2::W`](W) writer structure"]
impl crate::Writable for Win3Mst2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_MST2 to value 0"]
impl crate::Resettable for Win3Mst2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `WIN3_MST1` reader"]
pub type R = crate::R<Win3Mst1Spec>;
#[doc = "Register `WIN3_MST1` writer"]
pub type W = crate::W<Win3Mst1Spec>;
#[doc = "Field `WIN3_MST1` reader - Win3 frame buffer memory start address1\n\n*must be alianed to 8byte address"]
pub type Win3Mst1R = crate::FieldReader<u32>;
#[doc = "Field `WIN3_MST1` writer - Win3 frame buffer memory start address1\n\n*must be alianed to 8byte address"]
pub type Win3Mst1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Win3 frame buffer memory start address1\n\n*must be alianed to 8byte address"]
    #[inline(always)]
    pub fn win3_mst1(&self) -> Win3Mst1R {
        Win3Mst1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Win3 frame buffer memory start address1\n\n*must be alianed to 8byte address"]
    #[inline(always)]
    #[must_use]
    pub fn win3_mst1(&mut self) -> Win3Mst1W<Win3Mst1Spec> {
        Win3Mst1W::new(self, 0)
    }
}
#[doc = "Win3 memory start address1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_mst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_mst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Mst1Spec;
impl crate::RegisterSpec for Win3Mst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_mst1::R`](R) reader structure"]
impl crate::Readable for Win3Mst1Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_mst1::W`](W) writer structure"]
impl crate::Writable for Win3Mst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_MST1 to value 0"]
impl crate::Resettable for Win3Mst1Spec {
    const RESET_VALUE: u32 = 0;
}

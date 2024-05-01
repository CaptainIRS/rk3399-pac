#[doc = "Register `WIN2_MST0` reader"]
pub type R = crate::R<Win2Mst0Spec>;
#[doc = "Register `WIN2_MST0` writer"]
pub type W = crate::W<Win2Mst0Spec>;
#[doc = "Field `WIN2_MST0` reader - Win2 frame buffer memory start address0\n\n*must be alianed to 8byte address"]
pub type Win2Mst0R = crate::FieldReader<u32>;
#[doc = "Field `WIN2_MST0` writer - Win2 frame buffer memory start address0\n\n*must be alianed to 8byte address"]
pub type Win2Mst0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Win2 frame buffer memory start address0\n\n*must be alianed to 8byte address"]
    #[inline(always)]
    pub fn win2_mst0(&self) -> Win2Mst0R {
        Win2Mst0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Win2 frame buffer memory start address0\n\n*must be alianed to 8byte address"]
    #[inline(always)]
    #[must_use]
    pub fn win2_mst0(&mut self) -> Win2Mst0W<Win2Mst0Spec> {
        Win2Mst0W::new(self, 0)
    }
}
#[doc = "Win2 memory start address0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_mst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_mst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Mst0Spec;
impl crate::RegisterSpec for Win2Mst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_mst0::R`](R) reader structure"]
impl crate::Readable for Win2Mst0Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_mst0::W`](W) writer structure"]
impl crate::Writable for Win2Mst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_MST0 to value 0"]
impl crate::Resettable for Win2Mst0Spec {
    const RESET_VALUE: u32 = 0;
}

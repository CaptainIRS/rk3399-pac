#[doc = "Register `LSC_YSIZE_67` reader"]
pub type R = crate::R<LscYsize67Spec>;
#[doc = "Register `LSC_YSIZE_67` writer"]
pub type W = crate::W<LscYsize67Spec>;
#[doc = "Field `y_sect_size_6` reader - sector size 6 in y-direction"]
pub type YSectSize6R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_6` writer - sector size 6 in y-direction"]
pub type YSectSize6W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `y_sect_size_7` reader - sector size 7 in y-direction"]
pub type YSectSize7R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_7` writer - sector size 7 in y-direction"]
pub type YSectSize7W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 6 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_6(&self) -> YSectSize6R {
        YSectSize6R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 7 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_7(&self) -> YSectSize7R {
        YSectSize7R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 6 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_6(&mut self) -> YSectSize6W<LscYsize67Spec> {
        YSectSize6W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 7 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_7(&mut self) -> YSectSize7W<LscYsize67Spec> {
        YSectSize7W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYsize67Spec;
impl crate::RegisterSpec for LscYsize67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ysize_67::R`](R) reader structure"]
impl crate::Readable for LscYsize67Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ysize_67::W`](W) writer structure"]
impl crate::Writable for LscYsize67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YSIZE_67 to value 0"]
impl crate::Resettable for LscYsize67Spec {
    const RESET_VALUE: u32 = 0;
}

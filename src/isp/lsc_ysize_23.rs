#[doc = "Register `LSC_YSIZE_23` reader"]
pub type R = crate::R<LscYsize23Spec>;
#[doc = "Register `LSC_YSIZE_23` writer"]
pub type W = crate::W<LscYsize23Spec>;
#[doc = "Field `y_sect_size_2` reader - sector size 2 in y-direction"]
pub type YSectSize2R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_2` writer - sector size 2 in y-direction"]
pub type YSectSize2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `y_sect_size_3` reader - sector size 3 in y-direction"]
pub type YSectSize3R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_3` writer - sector size 3 in y-direction"]
pub type YSectSize3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 2 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_2(&self) -> YSectSize2R {
        YSectSize2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 3 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_3(&self) -> YSectSize3R {
        YSectSize3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 2 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_2(&mut self) -> YSectSize2W<LscYsize23Spec> {
        YSectSize2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 3 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_3(&mut self) -> YSectSize3W<LscYsize23Spec> {
        YSectSize3W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYsize23Spec;
impl crate::RegisterSpec for LscYsize23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ysize_23::R`](R) reader structure"]
impl crate::Readable for LscYsize23Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ysize_23::W`](W) writer structure"]
impl crate::Writable for LscYsize23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YSIZE_23 to value 0"]
impl crate::Resettable for LscYsize23Spec {
    const RESET_VALUE: u32 = 0;
}

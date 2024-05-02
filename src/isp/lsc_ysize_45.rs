#[doc = "Register `LSC_YSIZE_45` reader"]
pub type R = crate::R<LscYsize45Spec>;
#[doc = "Register `LSC_YSIZE_45` writer"]
pub type W = crate::W<LscYsize45Spec>;
#[doc = "Field `y_sect_size_4` reader - sector size 4 in y-direction"]
pub type YSectSize4R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_4` writer - sector size 4 in y-direction"]
pub type YSectSize4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `y_sect_size_5` reader - sector size 5 in y-direction"]
pub type YSectSize5R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_5` writer - sector size 5 in y-direction"]
pub type YSectSize5W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 4 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_4(&self) -> YSectSize4R {
        YSectSize4R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 5 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_5(&self) -> YSectSize5R {
        YSectSize5R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 4 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_4(&mut self) -> YSectSize4W<LscYsize45Spec> {
        YSectSize4W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 5 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_5(&mut self) -> YSectSize5W<LscYsize45Spec> {
        YSectSize5W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYsize45Spec;
impl crate::RegisterSpec for LscYsize45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ysize_45::R`](R) reader structure"]
impl crate::Readable for LscYsize45Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ysize_45::W`](W) writer structure"]
impl crate::Writable for LscYsize45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YSIZE_45 to value 0"]
impl crate::Resettable for LscYsize45Spec {
    const RESET_VALUE: u32 = 0;
}

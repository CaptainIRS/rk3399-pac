#[doc = "Register `LSC_YSIZE_01` reader"]
pub type R = crate::R<LscYsize01Spec>;
#[doc = "Register `LSC_YSIZE_01` writer"]
pub type W = crate::W<LscYsize01Spec>;
#[doc = "Field `y_sect_size_0` reader - sector size 0 in y-direction"]
pub type YSectSize0R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_0` writer - sector size 0 in y-direction"]
pub type YSectSize0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `y_sect_size_1` reader - sector size 1 in y-direction"]
pub type YSectSize1R = crate::FieldReader<u16>;
#[doc = "Field `y_sect_size_1` writer - sector size 1 in y-direction"]
pub type YSectSize1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 0 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_0(&self) -> YSectSize0R {
        YSectSize0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 1 in y-direction"]
    #[inline(always)]
    pub fn y_sect_size_1(&self) -> YSectSize1R {
        YSectSize1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 0 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_0(&mut self) -> YSectSize0W<LscYsize01Spec> {
        YSectSize0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 1 in y-direction"]
    #[inline(always)]
    #[must_use]
    pub fn y_sect_size_1(&mut self) -> YSectSize1W<LscYsize01Spec> {
        YSectSize1W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 8 in y direction. \n\n\n\nThe sector size in y-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ysize_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ysize_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscYsize01Spec;
impl crate::RegisterSpec for LscYsize01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ysize_01::R`](R) reader structure"]
impl crate::Readable for LscYsize01Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ysize_01::W`](W) writer structure"]
impl crate::Writable for LscYsize01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_YSIZE_01 to value 0"]
impl crate::Resettable for LscYsize01Spec {
    const RESET_VALUE: u32 = 0;
}

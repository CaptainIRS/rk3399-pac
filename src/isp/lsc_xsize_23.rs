#[doc = "Register `LSC_XSIZE_23` reader"]
pub type R = crate::R<LscXsize23Spec>;
#[doc = "Register `LSC_XSIZE_23` writer"]
pub type W = crate::W<LscXsize23Spec>;
#[doc = "Field `x_sect_size_2` reader - sector size 2 in x-direction"]
pub type XSectSize2R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_2` writer - sector size 2 in x-direction"]
pub type XSectSize2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `x_sect_size_3` reader - sector size 3 in x-direction"]
pub type XSectSize3R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_3` writer - sector size 3 in x-direction"]
pub type XSectSize3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 2 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_2(&self) -> XSectSize2R {
        XSectSize2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 3 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_3(&self) -> XSectSize3R {
        XSectSize3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 2 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_2(&mut self) -> XSectSize2W<LscXsize23Spec> {
        XSectSize2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 3 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_3(&mut self) -> XSectSize3W<LscXsize23Spec> {
        XSectSize3W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXsize23Spec;
impl crate::RegisterSpec for LscXsize23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xsize_23::R`](R) reader structure"]
impl crate::Readable for LscXsize23Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xsize_23::W`](W) writer structure"]
impl crate::Writable for LscXsize23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XSIZE_23 to value 0"]
impl crate::Resettable for LscXsize23Spec {
    const RESET_VALUE: u32 = 0;
}

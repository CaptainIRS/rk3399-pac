#[doc = "Register `LSC_XSIZE_67` reader"]
pub type R = crate::R<LscXsize67Spec>;
#[doc = "Register `LSC_XSIZE_67` writer"]
pub type W = crate::W<LscXsize67Spec>;
#[doc = "Field `x_sect_size_6` reader - sector size 6 in x-direction"]
pub type XSectSize6R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_6` writer - sector size 6 in x-direction"]
pub type XSectSize6W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `x_sect_size_7` reader - sector size 7 in x-direction"]
pub type XSectSize7R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_7` writer - sector size 7 in x-direction"]
pub type XSectSize7W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 6 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_6(&self) -> XSectSize6R {
        XSectSize6R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 7 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_7(&self) -> XSectSize7R {
        XSectSize7R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 6 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_6(&mut self) -> XSectSize6W<LscXsize67Spec> {
        XSectSize6W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 7 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_7(&mut self) -> XSectSize7W<LscXsize67Spec> {
        XSectSize7W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXsize67Spec;
impl crate::RegisterSpec for LscXsize67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xsize_67::R`](R) reader structure"]
impl crate::Readable for LscXsize67Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xsize_67::W`](W) writer structure"]
impl crate::Writable for LscXsize67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XSIZE_67 to value 0"]
impl crate::Resettable for LscXsize67Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `LSC_XSIZE_45` reader"]
pub type R = crate::R<LscXsize45Spec>;
#[doc = "Register `LSC_XSIZE_45` writer"]
pub type W = crate::W<LscXsize45Spec>;
#[doc = "Field `x_sect_size_4` reader - sector size 4in x-direction"]
pub type XSectSize4R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_4` writer - sector size 4in x-direction"]
pub type XSectSize4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `x_sect_size_5` reader - sector size 5 in x-direction"]
pub type XSectSize5R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_5` writer - sector size 5 in x-direction"]
pub type XSectSize5W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 4in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_4(&self) -> XSectSize4R {
        XSectSize4R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 5 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_5(&self) -> XSectSize5R {
        XSectSize5R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 4in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_4(&mut self) -> XSectSize4W<LscXsize45Spec> {
        XSectSize4W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 5 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_5(&mut self) -> XSectSize5W<LscXsize45Spec> {
        XSectSize5W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: minimum sector size is 10 in x direction \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXsize45Spec;
impl crate::RegisterSpec for LscXsize45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xsize_45::R`](R) reader structure"]
impl crate::Readable for LscXsize45Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xsize_45::W`](W) writer structure"]
impl crate::Writable for LscXsize45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XSIZE_45 to value 0"]
impl crate::Resettable for LscXsize45Spec {
    const RESET_VALUE: u32 = 0;
}

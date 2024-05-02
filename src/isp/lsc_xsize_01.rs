#[doc = "Register `LSC_XSIZE_01` reader"]
pub type R = crate::R<LscXsize01Spec>;
#[doc = "Register `LSC_XSIZE_01` writer"]
pub type W = crate::W<LscXsize01Spec>;
#[doc = "Field `x_sect_size_0` reader - sector size 0 in x-direction"]
pub type XSectSize0R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_0` writer - sector size 0 in x-direction"]
pub type XSectSize0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `x_sect_size_1` reader - sector size 1 in x-direction"]
pub type XSectSize1R = crate::FieldReader<u16>;
#[doc = "Field `x_sect_size_1` writer - sector size 1 in x-direction"]
pub type XSectSize1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sector size 0 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_0(&self) -> XSectSize0R {
        XSectSize0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - sector size 1 in x-direction"]
    #[inline(always)]
    pub fn x_sect_size_1(&self) -> XSectSize1R {
        XSectSize1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sector size 0 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_0(&mut self) -> XSectSize0W<LscXsize01Spec> {
        XSectSize0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - sector size 1 in x-direction"]
    #[inline(always)]
    #[must_use]
    pub fn x_sect_size_1(&mut self) -> XSectSize1W<LscXsize01Spec> {
        XSectSize1W::new(self, 16)
    }
}
#[doc = "Size table\n\nNote: The sector size in x-direction must be greater than 12 pixels. The sum of the sector \n\nsizes in x- direction must be 'picture width / 2'. The sum of the sector sizes in y-direction must \n\nbe 'picture height / 2'. No interrupt is generated if above requirements are not fulfilled and the \n\nbehaviour of the hardware cannot be predicted. \n\n\n\nThe sector size in x-direction was defined to be 9 bits for preliminary ISP versions. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_xsize_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_xsize_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscXsize01Spec;
impl crate::RegisterSpec for LscXsize01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_xsize_01::R`](R) reader structure"]
impl crate::Readable for LscXsize01Spec {}
#[doc = "`write(|w| ..)` method takes [`lsc_xsize_01::W`](W) writer structure"]
impl crate::Writable for LscXsize01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_XSIZE_01 to value 0"]
impl crate::Resettable for LscXsize01Spec {
    const RESET_VALUE: u32 = 0;
}

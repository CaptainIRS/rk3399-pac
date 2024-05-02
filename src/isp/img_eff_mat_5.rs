#[doc = "Register `IMG_EFF_MAT_5` reader"]
pub type R = crate::R<ImgEffMat5Spec>;
#[doc = "Register `IMG_EFF_MAT_5` writer"]
pub type W = crate::W<ImgEffMat5Spec>;
#[doc = "Field `sket_coef_32` reader - third line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type SketCoef32R = crate::FieldReader;
#[doc = "Field `sket_coef_32` writer - third line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type SketCoef32W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef32En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef32En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef32En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_32_en` reader - "]
pub type SketCoef32EnR = crate::BitReader<SketCoef32En>;
impl SketCoef32EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef32En {
        match self.bits {
            false => SketCoef32En::B0,
            true => SketCoef32En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef32En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef32En::B1
    }
}
#[doc = "Field `sket_coef_32_en` writer - "]
pub type SketCoef32EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef32En>;
impl<'a, REG> SketCoef32EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef32En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef32En::B1)
    }
}
#[doc = "Field `sket_coef_33` reader - third line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef33R = crate::FieldReader;
#[doc = "Field `sket_coef_33` writer - third line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef33W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef33En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef33En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef33En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_33_en` reader - "]
pub type SketCoef33EnR = crate::BitReader<SketCoef33En>;
impl SketCoef33EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef33En {
        match self.bits {
            false => SketCoef33En::B0,
            true => SketCoef33En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef33En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef33En::B1
    }
}
#[doc = "Field `sket_coef_33_en` writer - "]
pub type SketCoef33EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef33En>;
impl<'a, REG> SketCoef33EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef33En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef33En::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - third line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_32(&self) -> SketCoef32R {
        SketCoef32R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sket_coef_32_en(&self) -> SketCoef32EnR {
        SketCoef32EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - third line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_33(&self) -> SketCoef33R {
        SketCoef33R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sket_coef_33_en(&self) -> SketCoef33EnR {
        SketCoef33EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - third line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_32(&mut self) -> SketCoef32W<ImgEffMat5Spec> {
        SketCoef32W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_32_en(&mut self) -> SketCoef32EnW<ImgEffMat5Spec> {
        SketCoef32EnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - third line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_33(&mut self) -> SketCoef33W<ImgEffMat5Spec> {
        SketCoef33W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_33_en(&mut self) -> SketCoef33EnW<ImgEffMat5Spec> {
        SketCoef33EnW::new(self, 7)
    }
}
#[doc = "3x3 matrix coefficients for sketch/sharpen effect (3)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffMat5Spec;
impl crate::RegisterSpec for ImgEffMat5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_mat_5::R`](R) reader structure"]
impl crate::Readable for ImgEffMat5Spec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_mat_5::W`](W) writer structure"]
impl crate::Writable for ImgEffMat5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_MAT_5 to value 0xcc"]
impl crate::Resettable for ImgEffMat5Spec {
    const RESET_VALUE: u32 = 0xcc;
}

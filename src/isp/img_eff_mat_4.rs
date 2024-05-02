#[doc = "Register `IMG_EFF_MAT_4` reader"]
pub type R = crate::R<ImgEffMat4Spec>;
#[doc = "Register `IMG_EFF_MAT_4` writer"]
pub type W = crate::W<ImgEffMat4Spec>;
#[doc = "Field `sket_coef_21` reader - second line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
pub type SketCoef21R = crate::FieldReader;
#[doc = "Field `sket_coef_21` writer - second line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
pub type SketCoef21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef21En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef21En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef21En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_21_en` reader - "]
pub type SketCoef21EnR = crate::BitReader<SketCoef21En>;
impl SketCoef21EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef21En {
        match self.bits {
            false => SketCoef21En::B0,
            true => SketCoef21En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef21En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef21En::B1
    }
}
#[doc = "Field `sket_coef_21_en` writer - "]
pub type SketCoef21EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef21En>;
impl<'a, REG> SketCoef21EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef21En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef21En::B1)
    }
}
#[doc = "Field `sket_coef_22` reader - second line, middle entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef22R = crate::FieldReader;
#[doc = "Field `sket_coef_22` writer - second line, middle entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef22W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef22En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef22En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef22En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_22_en` reader - "]
pub type SketCoef22EnR = crate::BitReader<SketCoef22En>;
impl SketCoef22EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef22En {
        match self.bits {
            false => SketCoef22En::B0,
            true => SketCoef22En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef22En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef22En::B1
    }
}
#[doc = "Field `sket_coef_22_en` writer - "]
pub type SketCoef22EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef22En>;
impl<'a, REG> SketCoef22EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef22En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef22En::B1)
    }
}
#[doc = "Field `sket_coef_23` reader - second line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef23R = crate::FieldReader;
#[doc = "Field `sket_coef_23` writer - second line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef23W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef23En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef23En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef23En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_23_en` reader - "]
pub type SketCoef23EnR = crate::BitReader<SketCoef23En>;
impl SketCoef23EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef23En {
        match self.bits {
            false => SketCoef23En::B0,
            true => SketCoef23En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef23En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef23En::B1
    }
}
#[doc = "Field `sket_coef_23_en` writer - "]
pub type SketCoef23EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef23En>;
impl<'a, REG> SketCoef23EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef23En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef23En::B1)
    }
}
#[doc = "Field `sket_coef_31` reader - third line, left entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
pub type SketCoef31R = crate::FieldReader;
#[doc = "Field `sket_coef_31` writer - third line, left entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
pub type SketCoef31W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sket_coef_31_en` reader - 0: entry not used (coefficient is\n\nzero) 1: entry used"]
pub type SketCoef31EnR = crate::BitReader;
#[doc = "Field `sket_coef_31_en` writer - 0: entry not used (coefficient is\n\nzero) 1: entry used"]
pub type SketCoef31EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - second line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_21(&self) -> SketCoef21R {
        SketCoef21R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sket_coef_21_en(&self) -> SketCoef21EnR {
        SketCoef21EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - second line, middle entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_22(&self) -> SketCoef22R {
        SketCoef22R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sket_coef_22_en(&self) -> SketCoef22EnR {
        SketCoef22EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - second line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_23(&self) -> SketCoef23R {
        SketCoef23R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sket_coef_23_en(&self) -> SketCoef23EnR {
        SketCoef23EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - third line, left entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
    #[inline(always)]
    pub fn sket_coef_31(&self) -> SketCoef31R {
        SketCoef31R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 0: entry not used (coefficient is\n\nzero) 1: entry used"]
    #[inline(always)]
    pub fn sket_coef_31_en(&self) -> SketCoef31EnR {
        SketCoef31EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - second line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_21(&mut self) -> SketCoef21W<ImgEffMat4Spec> {
        SketCoef21W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_21_en(&mut self) -> SketCoef21EnW<ImgEffMat4Spec> {
        SketCoef21EnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - second line, middle entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_22(&mut self) -> SketCoef22W<ImgEffMat4Spec> {
        SketCoef22W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_22_en(&mut self) -> SketCoef22EnW<ImgEffMat4Spec> {
        SketCoef22EnW::new(self, 7)
    }
    #[doc = "Bits 8:10 - second line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_23(&mut self) -> SketCoef23W<ImgEffMat4Spec> {
        SketCoef23W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_23_en(&mut self) -> SketCoef23EnW<ImgEffMat4Spec> {
        SketCoef23EnW::new(self, 11)
    }
    #[doc = "Bits 12:14 - third line, left entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_31(&mut self) -> SketCoef31W<ImgEffMat4Spec> {
        SketCoef31W::new(self, 12)
    }
    #[doc = "Bit 15 - 0: entry not used (coefficient is\n\nzero) 1: entry used"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_31_en(&mut self) -> SketCoef31EnW<ImgEffMat4Spec> {
        SketCoef31EnW::new(self, 15)
    }
}
#[doc = "3x3 matrix coefficients for sketch/sharpen effect (2)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffMat4Spec;
impl crate::RegisterSpec for ImgEffMat4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_mat_4::R`](R) reader structure"]
impl crate::Readable for ImgEffMat4Spec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_mat_4::W`](W) writer structure"]
impl crate::Writable for ImgEffMat4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_MAT_4 to value 0xccbc"]
impl crate::Resettable for ImgEffMat4Spec {
    const RESET_VALUE: u32 = 0xccbc;
}

#[doc = "Register `IMG_EFF_MAT_2` reader"]
pub type R = crate::R<ImgEffMat2Spec>;
#[doc = "Register `IMG_EFF_MAT_2` writer"]
pub type W = crate::W<ImgEffMat2Spec>;
#[doc = "Field `emb_coef_22` reader - second line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
pub type EmbCoef22R = crate::FieldReader;
#[doc = "Field `emb_coef_22` writer - second line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
pub type EmbCoef22W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef22En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef22En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef22En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_22_en` reader - "]
pub type EmbCoef22EnR = crate::BitReader<EmbCoef22En>;
impl EmbCoef22EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef22En {
        match self.bits {
            false => EmbCoef22En::B0,
            true => EmbCoef22En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef22En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef22En::B1
    }
}
#[doc = "Field `emb_coef_22_en` writer - "]
pub type EmbCoef22EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef22En>;
impl<'a, REG> EmbCoef22EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef22En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef22En::B1)
    }
}
#[doc = "Field `emb_coef_23` reader - second line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef23R = crate::FieldReader;
#[doc = "Field `emb_coef_23` writer - second line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef23W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef23En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef23En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef23En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_23_en` reader - "]
pub type EmbCoef23EnR = crate::BitReader<EmbCoef23En>;
impl EmbCoef23EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef23En {
        match self.bits {
            false => EmbCoef23En::B0,
            true => EmbCoef23En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef23En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef23En::B1
    }
}
#[doc = "Field `emb_coef_23_en` writer - "]
pub type EmbCoef23EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef23En>;
impl<'a, REG> EmbCoef23EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef23En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef23En::B1)
    }
}
#[doc = "Field `emb_coef_31` reader - third line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type EmbCoef31R = crate::FieldReader;
#[doc = "Field `emb_coef_31` writer - third line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type EmbCoef31W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef31En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef31En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef31En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_31_en` reader - "]
pub type EmbCoef31EnR = crate::BitReader<EmbCoef31En>;
impl EmbCoef31EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef31En {
        match self.bits {
            false => EmbCoef31En::B0,
            true => EmbCoef31En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef31En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef31En::B1
    }
}
#[doc = "Field `emb_coef_31_en` writer - "]
pub type EmbCoef31EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef31En>;
impl<'a, REG> EmbCoef31EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef31En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef31En::B1)
    }
}
#[doc = "Field `emb_coef_32` reader - third line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef32R = crate::FieldReader;
#[doc = "Field `emb_coef_32` writer - third line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef32W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef32En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef32En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef32En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_32_en` reader - "]
pub type EmbCoef32EnR = crate::BitReader<EmbCoef32En>;
impl EmbCoef32EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef32En {
        match self.bits {
            false => EmbCoef32En::B0,
            true => EmbCoef32En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef32En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef32En::B1
    }
}
#[doc = "Field `emb_coef_32_en` writer - "]
pub type EmbCoef32EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef32En>;
impl<'a, REG> EmbCoef32EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef32En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef32En::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - second line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
    #[inline(always)]
    pub fn emb_coef_22(&self) -> EmbCoef22R {
        EmbCoef22R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emb_coef_22_en(&self) -> EmbCoef22EnR {
        EmbCoef22EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - second line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_23(&self) -> EmbCoef23R {
        EmbCoef23R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn emb_coef_23_en(&self) -> EmbCoef23EnR {
        EmbCoef23EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - third line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_31(&self) -> EmbCoef31R {
        EmbCoef31R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn emb_coef_31_en(&self) -> EmbCoef31EnR {
        EmbCoef31EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - third line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_32(&self) -> EmbCoef32R {
        EmbCoef32R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn emb_coef_32_en(&self) -> EmbCoef32EnR {
        EmbCoef32EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - second line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_22(&mut self) -> EmbCoef22W<ImgEffMat2Spec> {
        EmbCoef22W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_22_en(&mut self) -> EmbCoef22EnW<ImgEffMat2Spec> {
        EmbCoef22EnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - second line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_23(&mut self) -> EmbCoef23W<ImgEffMat2Spec> {
        EmbCoef23W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_23_en(&mut self) -> EmbCoef23EnW<ImgEffMat2Spec> {
        EmbCoef23EnW::new(self, 7)
    }
    #[doc = "Bits 8:10 - third line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_31(&mut self) -> EmbCoef31W<ImgEffMat2Spec> {
        EmbCoef31W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_31_en(&mut self) -> EmbCoef31EnW<ImgEffMat2Spec> {
        EmbCoef31EnW::new(self, 11)
    }
    #[doc = "Bits 12:14 - third line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_32(&mut self) -> EmbCoef32W<ImgEffMat2Spec> {
        EmbCoef32W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_32_en(&mut self) -> EmbCoef32EnW<ImgEffMat2Spec> {
        EmbCoef32EnW::new(self, 15)
    }
}
#[doc = "3x3 matrix coefficients for emboss effect (2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffMat2Spec;
impl crate::RegisterSpec for ImgEffMat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_mat_2::R`](R) reader structure"]
impl crate::Readable for ImgEffMat2Spec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_mat_2::W`](W) writer structure"]
impl crate::Writable for ImgEffMat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_MAT_2 to value 0xc0c0"]
impl crate::Resettable for ImgEffMat2Spec {
    const RESET_VALUE: u32 = 0xc0c0;
}

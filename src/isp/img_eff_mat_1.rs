#[doc = "Register `IMG_EFF_MAT_1` reader"]
pub type R = crate::R<ImgEffMat1Spec>;
#[doc = "Register `IMG_EFF_MAT_1` writer"]
pub type W = crate::W<ImgEffMat1Spec>;
#[doc = "Field `emb_coef_11` reader - first line, left entry of 3x3 emboss effect\n\nmatrix, 2 bit for coefficient, one sign bit."]
pub type EmbCoef11R = crate::FieldReader;
#[doc = "Field `emb_coef_11` writer - first line, left entry of 3x3 emboss effect\n\nmatrix, 2 bit for coefficient, one sign bit."]
pub type EmbCoef11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef11En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef11En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef11En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_11_en` reader - "]
pub type EmbCoef11EnR = crate::BitReader<EmbCoef11En>;
impl EmbCoef11EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef11En {
        match self.bits {
            false => EmbCoef11En::B0,
            true => EmbCoef11En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef11En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef11En::B1
    }
}
#[doc = "Field `emb_coef_11_en` writer - "]
pub type EmbCoef11EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef11En>;
impl<'a, REG> EmbCoef11EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef11En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef11En::B1)
    }
}
#[doc = "Field `emb_coef_12` reader - first line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef12R = crate::FieldReader;
#[doc = "Field `emb_coef_12` writer - first line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef12En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef12En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef12En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_12_en` reader - "]
pub type EmbCoef12EnR = crate::BitReader<EmbCoef12En>;
impl EmbCoef12EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef12En {
        match self.bits {
            false => EmbCoef12En::B0,
            true => EmbCoef12En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef12En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef12En::B1
    }
}
#[doc = "Field `emb_coef_12_en` writer - "]
pub type EmbCoef12EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef12En>;
impl<'a, REG> EmbCoef12EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef12En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef12En::B1)
    }
}
#[doc = "Field `emb_coef_13` reader - first line, right entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type EmbCoef13R = crate::FieldReader;
#[doc = "Field `emb_coef_13` writer - first line, right entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type EmbCoef13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef13En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef13En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef13En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_13_en` reader - "]
pub type EmbCoef13EnR = crate::BitReader<EmbCoef13En>;
impl EmbCoef13EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef13En {
        match self.bits {
            false => EmbCoef13En::B0,
            true => EmbCoef13En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef13En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef13En::B1
    }
}
#[doc = "Field `emb_coef_13_en` writer - "]
pub type EmbCoef13EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef13En>;
impl<'a, REG> EmbCoef13EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef13En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef13En::B1)
    }
}
#[doc = "Field `emb_coef_21` reader - second line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type EmbCoef21R = crate::FieldReader;
#[doc = "Field `emb_coef_21` writer - second line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type EmbCoef21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef21En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef21En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef21En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_21_en` reader - "]
pub type EmbCoef21EnR = crate::BitReader<EmbCoef21En>;
impl EmbCoef21EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef21En {
        match self.bits {
            false => EmbCoef21En::B0,
            true => EmbCoef21En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef21En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef21En::B1
    }
}
#[doc = "Field `emb_coef_21_en` writer - "]
pub type EmbCoef21EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef21En>;
impl<'a, REG> EmbCoef21EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef21En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef21En::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - first line, left entry of 3x3 emboss effect\n\nmatrix, 2 bit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_11(&self) -> EmbCoef11R {
        EmbCoef11R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emb_coef_11_en(&self) -> EmbCoef11EnR {
        EmbCoef11EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - first line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_12(&self) -> EmbCoef12R {
        EmbCoef12R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn emb_coef_12_en(&self) -> EmbCoef12EnR {
        EmbCoef12EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - first line, right entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_13(&self) -> EmbCoef13R {
        EmbCoef13R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn emb_coef_13_en(&self) -> EmbCoef13EnR {
        EmbCoef13EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - second line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_21(&self) -> EmbCoef21R {
        EmbCoef21R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn emb_coef_21_en(&self) -> EmbCoef21EnR {
        EmbCoef21EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - first line, left entry of 3x3 emboss effect\n\nmatrix, 2 bit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_11(&mut self) -> EmbCoef11W<ImgEffMat1Spec> {
        EmbCoef11W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_11_en(&mut self) -> EmbCoef11EnW<ImgEffMat1Spec> {
        EmbCoef11EnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - first line, middle entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_12(&mut self) -> EmbCoef12W<ImgEffMat1Spec> {
        EmbCoef12W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_12_en(&mut self) -> EmbCoef12EnW<ImgEffMat1Spec> {
        EmbCoef12EnW::new(self, 7)
    }
    #[doc = "Bits 8:10 - first line, right entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_13(&mut self) -> EmbCoef13W<ImgEffMat1Spec> {
        EmbCoef13W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_13_en(&mut self) -> EmbCoef13EnW<ImgEffMat1Spec> {
        EmbCoef13EnW::new(self, 11)
    }
    #[doc = "Bits 12:14 - second line, left entry of 3x3 emboss effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_21(&mut self) -> EmbCoef21W<ImgEffMat1Spec> {
        EmbCoef21W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_21_en(&mut self) -> EmbCoef21EnW<ImgEffMat1Spec> {
        EmbCoef21EnW::new(self, 15)
    }
}
#[doc = "3x3 matrix coefficients for emboss effect (1)\n\nNote: possible values for coefficients: 000 (1), 001 (2), 010 (4), 011 (8),100 (-1), 101 \n\n(-2), 110 (-4), 111 (-8) \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffMat1Spec;
impl crate::RegisterSpec for ImgEffMat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_mat_1::R`](R) reader structure"]
impl crate::Readable for ImgEffMat1Spec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_mat_1::W`](W) writer structure"]
impl crate::Writable for ImgEffMat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_MAT_1 to value 0x8089"]
impl crate::Resettable for ImgEffMat1Spec {
    const RESET_VALUE: u32 = 0x8089;
}

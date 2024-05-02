#[doc = "Register `IMG_EFF_MAT_3` reader"]
pub type R = crate::R<ImgEffMat3Spec>;
#[doc = "Register `IMG_EFF_MAT_3` writer"]
pub type W = crate::W<ImgEffMat3Spec>;
#[doc = "Field `emb_coef_33` reader - third line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef33R = crate::FieldReader;
#[doc = "Field `emb_coef_33` writer - third line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type EmbCoef33W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbCoef33En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<EmbCoef33En> for bool {
    #[inline(always)]
    fn from(variant: EmbCoef33En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `emb_coef_33_en` reader - "]
pub type EmbCoef33EnR = crate::BitReader<EmbCoef33En>;
impl EmbCoef33EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmbCoef33En {
        match self.bits {
            false => EmbCoef33En::B0,
            true => EmbCoef33En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmbCoef33En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmbCoef33En::B1
    }
}
#[doc = "Field `emb_coef_33_en` writer - "]
pub type EmbCoef33EnW<'a, REG> = crate::BitWriter<'a, REG, EmbCoef33En>;
impl<'a, REG> EmbCoef33EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef33En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmbCoef33En::B1)
    }
}
#[doc = "Field `sket_coef_11` reader - first line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
pub type SketCoef11R = crate::FieldReader;
#[doc = "Field `sket_coef_11` writer - first line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
pub type SketCoef11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef11En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef11En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef11En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_11_en` reader - "]
pub type SketCoef11EnR = crate::BitReader<SketCoef11En>;
impl SketCoef11EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef11En {
        match self.bits {
            false => SketCoef11En::B0,
            true => SketCoef11En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef11En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef11En::B1
    }
}
#[doc = "Field `sket_coef_11_en` writer - "]
pub type SketCoef11EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef11En>;
impl<'a, REG> SketCoef11EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef11En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef11En::B1)
    }
}
#[doc = "Field `sket_coef_12` reader - first line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type SketCoef12R = crate::FieldReader;
#[doc = "Field `sket_coef_12` writer - first line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
pub type SketCoef12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef12En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef12En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef12En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_12_en` reader - "]
pub type SketCoef12EnR = crate::BitReader<SketCoef12En>;
impl SketCoef12EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef12En {
        match self.bits {
            false => SketCoef12En::B0,
            true => SketCoef12En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef12En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef12En::B1
    }
}
#[doc = "Field `sket_coef_12_en` writer - "]
pub type SketCoef12EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef12En>;
impl<'a, REG> SketCoef12EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef12En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef12En::B1)
    }
}
#[doc = "Field `sket_coef_13` reader - first line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef13R = crate::FieldReader;
#[doc = "Field `sket_coef_13` writer - first line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
pub type SketCoef13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketCoef13En {
    #[doc = "0: entry not used (coefficient is zero)"]
    B0 = 0,
    #[doc = "1: entry used"]
    B1 = 1,
}
impl From<SketCoef13En> for bool {
    #[inline(always)]
    fn from(variant: SketCoef13En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sket_coef_13_en` reader - "]
pub type SketCoef13EnR = crate::BitReader<SketCoef13En>;
impl SketCoef13EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SketCoef13En {
        match self.bits {
            false => SketCoef13En::B0,
            true => SketCoef13En::B1,
        }
    }
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SketCoef13En::B0
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SketCoef13En::B1
    }
}
#[doc = "Field `sket_coef_13_en` writer - "]
pub type SketCoef13EnW<'a, REG> = crate::BitWriter<'a, REG, SketCoef13En>;
impl<'a, REG> SketCoef13EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "entry not used (coefficient is zero)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef13En::B0)
    }
    #[doc = "entry used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SketCoef13En::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - third line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn emb_coef_33(&self) -> EmbCoef33R {
        EmbCoef33R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emb_coef_33_en(&self) -> EmbCoef33EnR {
        EmbCoef33EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - first line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_11(&self) -> SketCoef11R {
        SketCoef11R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sket_coef_11_en(&self) -> SketCoef11EnR {
        SketCoef11EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - first line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_12(&self) -> SketCoef12R {
        SketCoef12R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sket_coef_12_en(&self) -> SketCoef12EnR {
        SketCoef12EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - first line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    pub fn sket_coef_13(&self) -> SketCoef13R {
        SketCoef13R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sket_coef_13_en(&self) -> SketCoef13EnR {
        SketCoef13EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - third line, right entry of 3x3 emboss effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_33(&mut self) -> EmbCoef33W<ImgEffMat3Spec> {
        EmbCoef33W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn emb_coef_33_en(&mut self) -> EmbCoef33EnW<ImgEffMat3Spec> {
        EmbCoef33EnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - first line, left entry of 3x3 sketch effect matrix, 2 bit for\n\ncoefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_11(&mut self) -> SketCoef11W<ImgEffMat3Spec> {
        SketCoef11W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_11_en(&mut self) -> SketCoef11EnW<ImgEffMat3Spec> {
        SketCoef11EnW::new(self, 7)
    }
    #[doc = "Bits 8:10 - first line, middle entry of 3x3 sketch effect matrix, 2\n\nbit for coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_12(&mut self) -> SketCoef12W<ImgEffMat3Spec> {
        SketCoef12W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_12_en(&mut self) -> SketCoef12EnW<ImgEffMat3Spec> {
        SketCoef12EnW::new(self, 11)
    }
    #[doc = "Bits 12:14 - first line, right entry of 3x3 sketch effect matrix, 2 bit\n\nfor coefficient, one sign bit."]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_13(&mut self) -> SketCoef13W<ImgEffMat3Spec> {
        SketCoef13W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn sket_coef_13_en(&mut self) -> SketCoef13EnW<ImgEffMat3Spec> {
        SketCoef13EnW::new(self, 15)
    }
}
#[doc = "3x3 matrix coefficients for emboss(3) effect / \n\n\n\nsketch/sharpen(1) effect\n\nNote: possible values for \n\n\n\ncoefficients: 000 (1), 001 (2), 010 \n\n\n\n(4), 011 (8), \n\n100 (-1), 101 (-2), 110 (-4), 111 (-8) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_mat_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_mat_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffMat3Spec;
impl crate::RegisterSpec for ImgEffMat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_mat_3::R`](R) reader structure"]
impl crate::Readable for ImgEffMat3Spec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_mat_3::W`](W) writer structure"]
impl crate::Writable for ImgEffMat3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_MAT_3 to value 0xcccd"]
impl crate::Resettable for ImgEffMat3Spec {
    const RESET_VALUE: u32 = 0xcccd;
}

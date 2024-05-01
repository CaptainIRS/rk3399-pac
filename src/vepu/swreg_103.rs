#[doc = "Register `SWREG_103` reader"]
pub type R = crate::R<Swreg103Spec>;
#[doc = "Register `SWREG_103` writer"]
pub type W = crate::W<Swreg103Spec>;
#[doc = "Field `ENC_EN` reader - encoder enable flag\n\nencoder enable"]
pub type EncEnR = crate::BitReader;
#[doc = "Field `ENC_EN` writer - encoder enable flag\n\nencoder enable"]
pub type EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "encoding format selected\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EncFmt {
    #[doc = "2: JPEG"]
    D2 = 2,
    #[doc = "3: H264"]
    D3 = 3,
}
impl From<EncFmt> for u8 {
    #[inline(always)]
    fn from(variant: EncFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EncFmt {
    type Ux = u8;
}
#[doc = "Field `ENC_FMT` reader - encoding format selected"]
pub type EncFmtR = crate::FieldReader<EncFmt>;
impl EncFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EncFmt> {
        match self.bits {
            2 => Some(EncFmt::D2),
            3 => Some(EncFmt::D3),
            _ => None,
        }
    }
    #[doc = "JPEG"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == EncFmt::D2
    }
    #[doc = "H264"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == EncFmt::D3
    }
}
#[doc = "Field `ENC_FMT` writer - encoding format selected"]
pub type EncFmtW<'a, REG> = crate::FieldWriter<'a, REG, 2, EncFmt>;
impl<'a, REG> EncFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "JPEG"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(EncFmt::D2)
    }
    #[doc = "H264"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(EncFmt::D3)
    }
}
#[doc = "frame type selected for current frame\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EncFrameType {
    #[doc = "0: INTER"]
    D0 = 0,
    #[doc = "1: INTRA(IDR)"]
    D1 = 1,
    #[doc = "2: MVC-INTER"]
    D2 = 2,
}
impl From<EncFrameType> for u8 {
    #[inline(always)]
    fn from(variant: EncFrameType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EncFrameType {
    type Ux = u8;
}
#[doc = "Field `ENC_FRAME_TYPE` reader - frame type selected for current frame"]
pub type EncFrameTypeR = crate::FieldReader<EncFrameType>;
impl EncFrameTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EncFrameType> {
        match self.bits {
            0 => Some(EncFrameType::D0),
            1 => Some(EncFrameType::D1),
            2 => Some(EncFrameType::D2),
            _ => None,
        }
    }
    #[doc = "INTER"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == EncFrameType::D0
    }
    #[doc = "INTRA(IDR)"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == EncFrameType::D1
    }
    #[doc = "MVC-INTER"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == EncFrameType::D2
    }
}
#[doc = "Field `ENC_FRAME_TYPE` writer - frame type selected for current frame"]
pub type EncFrameTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, EncFrameType>;
impl<'a, REG> EncFrameTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "INTER"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(EncFrameType::D0)
    }
    #[doc = "INTRA(IDR)"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(EncFrameType::D1)
    }
    #[doc = "MVC-INTER"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(EncFrameType::D2)
    }
}
#[doc = "Field `ENC_WIDTH` reader - the encoder width\n\nlum width (macroblock unit)\n\nH264:\n\nrange : 9~255\n\nJPEG:\n\nrange : 6~511"]
pub type EncWidthR = crate::FieldReader<u16>;
#[doc = "Field `ENC_WIDTH` writer - the encoder width\n\nlum width (macroblock unit)\n\nH264:\n\nrange : 9~255\n\nJPEG:\n\nrange : 6~511"]
pub type EncWidthW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ENC_HEIGHT` reader - encoderd height\n\nlum height (macroblock unit)\n\nH264: \\[6..255\\]\n\nJPEG: \\[6..511\\]"]
pub type EncHeightR = crate::FieldReader<u16>;
#[doc = "Field `ENC_HEIGHT` writer - encoderd height\n\nlum height (macroblock unit)\n\nH264: \\[6..255\\]\n\nJPEG: \\[6..511\\]"]
pub type EncHeightW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - encoder enable flag\n\nencoder enable"]
    #[inline(always)]
    pub fn enc_en(&self) -> EncEnR {
        EncEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - encoding format selected"]
    #[inline(always)]
    pub fn enc_fmt(&self) -> EncFmtR {
        EncFmtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - frame type selected for current frame"]
    #[inline(always)]
    pub fn enc_frame_type(&self) -> EncFrameTypeR {
        EncFrameTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:16 - the encoder width\n\nlum width (macroblock unit)\n\nH264:\n\nrange : 9~255\n\nJPEG:\n\nrange : 6~511"]
    #[inline(always)]
    pub fn enc_width(&self) -> EncWidthR {
        EncWidthR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - encoderd height\n\nlum height (macroblock unit)\n\nH264: \\[6..255\\]\n\nJPEG: \\[6..511\\]"]
    #[inline(always)]
    pub fn enc_height(&self) -> EncHeightR {
        EncHeightR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - encoder enable flag\n\nencoder enable"]
    #[inline(always)]
    #[must_use]
    pub fn enc_en(&mut self) -> EncEnW<Swreg103Spec> {
        EncEnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - encoding format selected"]
    #[inline(always)]
    #[must_use]
    pub fn enc_fmt(&mut self) -> EncFmtW<Swreg103Spec> {
        EncFmtW::new(self, 4)
    }
    #[doc = "Bits 6:7 - frame type selected for current frame"]
    #[inline(always)]
    #[must_use]
    pub fn enc_frame_type(&mut self) -> EncFrameTypeW<Swreg103Spec> {
        EncFrameTypeW::new(self, 6)
    }
    #[doc = "Bits 8:16 - the encoder width\n\nlum width (macroblock unit)\n\nH264:\n\nrange : 9~255\n\nJPEG:\n\nrange : 6~511"]
    #[inline(always)]
    #[must_use]
    pub fn enc_width(&mut self) -> EncWidthW<Swreg103Spec> {
        EncWidthW::new(self, 8)
    }
    #[doc = "Bits 20:28 - encoderd height\n\nlum height (macroblock unit)\n\nH264: \\[6..255\\]\n\nJPEG: \\[6..511\\]"]
    #[inline(always)]
    #[must_use]
    pub fn enc_height(&mut self) -> EncHeightW<Swreg103Spec> {
        EncHeightW::new(self, 20)
    }
}
#[doc = "encoder start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_103::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_103::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg103Spec;
impl crate::RegisterSpec for Swreg103Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_103::R`](R) reader structure"]
impl crate::Readable for Swreg103Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_103::W`](W) writer structure"]
impl crate::Writable for Swreg103Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_103 to value 0"]
impl crate::Resettable for Swreg103Spec {
    const RESET_VALUE: u32 = 0;
}

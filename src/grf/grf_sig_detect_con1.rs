#[doc = "Register `GRF_SIG_DETECT_CON1` reader"]
pub type R = crate::R<GrfSigDetectCon1Spec>;
#[doc = "Register `GRF_SIG_DETECT_CON1` writer"]
pub type W = crate::W<GrfSigDetectCon1Spec>;
#[doc = "filter time select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Otg0IdFilterTimeSel {
    #[doc = "0: 5ms"]
    B00 = 0,
    #[doc = "1: 15ms"]
    B01 = 1,
    #[doc = "2: 35ms"]
    B10 = 2,
    #[doc = "3: 50ms"]
    B11 = 3,
}
impl From<Otg0IdFilterTimeSel> for u8 {
    #[inline(always)]
    fn from(variant: Otg0IdFilterTimeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Otg0IdFilterTimeSel {
    type Ux = u8;
}
#[doc = "Field `OTG0_ID_FILTER_TIME_SEL` reader - filter time select"]
pub type Otg0IdFilterTimeSelR = crate::FieldReader<Otg0IdFilterTimeSel>;
impl Otg0IdFilterTimeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Otg0IdFilterTimeSel {
        match self.bits {
            0 => Otg0IdFilterTimeSel::B00,
            1 => Otg0IdFilterTimeSel::B01,
            2 => Otg0IdFilterTimeSel::B10,
            3 => Otg0IdFilterTimeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "5ms"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Otg0IdFilterTimeSel::B00
    }
    #[doc = "15ms"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Otg0IdFilterTimeSel::B01
    }
    #[doc = "35ms"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Otg0IdFilterTimeSel::B10
    }
    #[doc = "50ms"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Otg0IdFilterTimeSel::B11
    }
}
#[doc = "Field `OTG0_ID_FILTER_TIME_SEL` writer - filter time select"]
pub type Otg0IdFilterTimeSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Otg0IdFilterTimeSel>;
impl<'a, REG> Otg0IdFilterTimeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5ms"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0IdFilterTimeSel::B00)
    }
    #[doc = "15ms"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0IdFilterTimeSel::B01)
    }
    #[doc = "35ms"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0IdFilterTimeSel::B10)
    }
    #[doc = "50ms"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0IdFilterTimeSel::B11)
    }
}
#[doc = "filter time select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Otg0LlinestateFilterTimeSel {
    #[doc = "0: 100us"]
    B00 = 0,
    #[doc = "1: 500us"]
    B01 = 1,
    #[doc = "2: 1ms"]
    B10 = 2,
    #[doc = "3: 10ms"]
    B11 = 3,
}
impl From<Otg0LlinestateFilterTimeSel> for u8 {
    #[inline(always)]
    fn from(variant: Otg0LlinestateFilterTimeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Otg0LlinestateFilterTimeSel {
    type Ux = u8;
}
#[doc = "Field `OTG0_LLINESTATE_FILTER_TIME_SEL` reader - filter time select"]
pub type Otg0LlinestateFilterTimeSelR = crate::FieldReader<Otg0LlinestateFilterTimeSel>;
impl Otg0LlinestateFilterTimeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Otg0LlinestateFilterTimeSel {
        match self.bits {
            0 => Otg0LlinestateFilterTimeSel::B00,
            1 => Otg0LlinestateFilterTimeSel::B01,
            2 => Otg0LlinestateFilterTimeSel::B10,
            3 => Otg0LlinestateFilterTimeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "100us"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Otg0LlinestateFilterTimeSel::B00
    }
    #[doc = "500us"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Otg0LlinestateFilterTimeSel::B01
    }
    #[doc = "1ms"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Otg0LlinestateFilterTimeSel::B10
    }
    #[doc = "10ms"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Otg0LlinestateFilterTimeSel::B11
    }
}
#[doc = "Field `OTG0_LLINESTATE_FILTER_TIME_SEL` writer - filter time select"]
pub type Otg0LlinestateFilterTimeSelW<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, Otg0LlinestateFilterTimeSel>;
impl<'a, REG> Otg0LlinestateFilterTimeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100us"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0LlinestateFilterTimeSel::B00)
    }
    #[doc = "500us"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0LlinestateFilterTimeSel::B01)
    }
    #[doc = "1ms"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0LlinestateFilterTimeSel::B10)
    }
    #[doc = "10ms"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Otg0LlinestateFilterTimeSel::B11)
    }
}
#[doc = "filter time select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Host0LlinestateFilterTimeSel {
    #[doc = "0: 100us"]
    B00 = 0,
    #[doc = "1: 500us"]
    B01 = 1,
    #[doc = "2: 1ms"]
    B10 = 2,
    #[doc = "3: 10ms"]
    B11 = 3,
}
impl From<Host0LlinestateFilterTimeSel> for u8 {
    #[inline(always)]
    fn from(variant: Host0LlinestateFilterTimeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Host0LlinestateFilterTimeSel {
    type Ux = u8;
}
#[doc = "Field `HOST0_LLINESTATE_FILTER_TIME_SEL` reader - filter time select"]
pub type Host0LlinestateFilterTimeSelR = crate::FieldReader<Host0LlinestateFilterTimeSel>;
impl Host0LlinestateFilterTimeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Host0LlinestateFilterTimeSel {
        match self.bits {
            0 => Host0LlinestateFilterTimeSel::B00,
            1 => Host0LlinestateFilterTimeSel::B01,
            2 => Host0LlinestateFilterTimeSel::B10,
            3 => Host0LlinestateFilterTimeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "100us"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Host0LlinestateFilterTimeSel::B00
    }
    #[doc = "500us"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Host0LlinestateFilterTimeSel::B01
    }
    #[doc = "1ms"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Host0LlinestateFilterTimeSel::B10
    }
    #[doc = "10ms"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Host0LlinestateFilterTimeSel::B11
    }
}
#[doc = "Field `HOST0_LLINESTATE_FILTER_TIME_SEL` writer - filter time select"]
pub type Host0LlinestateFilterTimeSelW<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, Host0LlinestateFilterTimeSel>;
impl<'a, REG> Host0LlinestateFilterTimeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100us"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Host0LlinestateFilterTimeSel::B00)
    }
    #[doc = "500us"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Host0LlinestateFilterTimeSel::B01)
    }
    #[doc = "1ms"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Host0LlinestateFilterTimeSel::B10)
    }
    #[doc = "10ms"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Host0LlinestateFilterTimeSel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:9 - filter time select"]
    #[inline(always)]
    pub fn otg0_id_filter_time_sel(&self) -> Otg0IdFilterTimeSelR {
        Otg0IdFilterTimeSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - filter time select"]
    #[inline(always)]
    pub fn otg0_llinestate_filter_time_sel(&self) -> Otg0LlinestateFilterTimeSelR {
        Otg0LlinestateFilterTimeSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - filter time select"]
    #[inline(always)]
    pub fn host0_llinestate_filter_time_sel(&self) -> Host0LlinestateFilterTimeSelR {
        Host0LlinestateFilterTimeSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:9 - filter time select"]
    #[inline(always)]
    #[must_use]
    pub fn otg0_id_filter_time_sel(&mut self) -> Otg0IdFilterTimeSelW<GrfSigDetectCon1Spec> {
        Otg0IdFilterTimeSelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - filter time select"]
    #[inline(always)]
    #[must_use]
    pub fn otg0_llinestate_filter_time_sel(
        &mut self,
    ) -> Otg0LlinestateFilterTimeSelW<GrfSigDetectCon1Spec> {
        Otg0LlinestateFilterTimeSelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - filter time select"]
    #[inline(always)]
    #[must_use]
    pub fn host0_llinestate_filter_time_sel(
        &mut self,
    ) -> Host0LlinestateFilterTimeSelW<GrfSigDetectCon1Spec> {
        Host0LlinestateFilterTimeSelW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSigDetectCon1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "Singal detect control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_sig_detect_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_sig_detect_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSigDetectCon1Spec;
impl crate::RegisterSpec for GrfSigDetectCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_sig_detect_con1::R`](R) reader structure"]
impl crate::Readable for GrfSigDetectCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_sig_detect_con1::W`](W) writer structure"]
impl crate::Writable for GrfSigDetectCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SIG_DETECT_CON1 to value 0"]
impl crate::Resettable for GrfSigDetectCon1Spec {
    const RESET_VALUE: u32 = 0;
}

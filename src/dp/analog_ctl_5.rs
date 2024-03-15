#[doc = "Register `ANALOG_CTL_5` reader"]
pub type R = crate::R<AnalogCtl5Spec>;
#[doc = "Register `ANALOG_CTL_5` writer"]
pub type W = crate::W<AnalogCtl5Spec>;
#[doc = "Ch0 post cursor2 setting: CH0_PC2_SEL: post cursor2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3Pc2SelBits3 {
    #[doc = "0: 0.15"]
    H0 = 0,
    #[doc = "1: 0.15"]
    H1 = 1,
    #[doc = "2: 0.15"]
    H2 = 2,
    #[doc = "3: 0.15"]
    H3 = 3,
}
impl From<Ch3Pc2SelBits3> for u8 {
    #[inline(always)]
    fn from(variant: Ch3Pc2SelBits3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3Pc2SelBits3 {
    type Ux = u8;
}
#[doc = "Field `CH3_PC2_SEL_BITS_3` reader - Ch0 post cursor2 setting: CH0_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits3R = crate::FieldReader<Ch3Pc2SelBits3>;
impl Ch3Pc2SelBits3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3Pc2SelBits3 {
        match self.bits {
            0 => Ch3Pc2SelBits3::H0,
            1 => Ch3Pc2SelBits3::H1,
            2 => Ch3Pc2SelBits3::H2,
            3 => Ch3Pc2SelBits3::H3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Ch3Pc2SelBits3::H0
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Ch3Pc2SelBits3::H1
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Ch3Pc2SelBits3::H2
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Ch3Pc2SelBits3::H3
    }
}
#[doc = "Field `CH3_PC2_SEL_BITS_3` writer - Ch0 post cursor2 setting: CH0_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits3W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ch3Pc2SelBits3>;
impl<'a, REG> Ch3Pc2SelBits3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits3::H0)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits3::H1)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits3::H2)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits3::H3)
    }
}
#[doc = "Ch1 post cursor2 setting: CH1_PC2_SEL: post cursor2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3Pc2SelBits2 {
    #[doc = "0: 0.15"]
    H0 = 0,
    #[doc = "1: 0.15"]
    H1 = 1,
    #[doc = "2: 0.15"]
    H2 = 2,
    #[doc = "3: 0.15"]
    H3 = 3,
}
impl From<Ch3Pc2SelBits2> for u8 {
    #[inline(always)]
    fn from(variant: Ch3Pc2SelBits2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3Pc2SelBits2 {
    type Ux = u8;
}
#[doc = "Field `CH3_PC2_SEL_BITS_2` reader - Ch1 post cursor2 setting: CH1_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits2R = crate::FieldReader<Ch3Pc2SelBits2>;
impl Ch3Pc2SelBits2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3Pc2SelBits2 {
        match self.bits {
            0 => Ch3Pc2SelBits2::H0,
            1 => Ch3Pc2SelBits2::H1,
            2 => Ch3Pc2SelBits2::H2,
            3 => Ch3Pc2SelBits2::H3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Ch3Pc2SelBits2::H0
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Ch3Pc2SelBits2::H1
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Ch3Pc2SelBits2::H2
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Ch3Pc2SelBits2::H3
    }
}
#[doc = "Field `CH3_PC2_SEL_BITS_2` writer - Ch1 post cursor2 setting: CH1_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ch3Pc2SelBits2>;
impl<'a, REG> Ch3Pc2SelBits2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits2::H0)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits2::H1)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits2::H2)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits2::H3)
    }
}
#[doc = "Ch2 post cursor2 setting: CH2_PC2_SEL: post cursor2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3Pc2SelBits1 {
    #[doc = "0: 0.15"]
    H0 = 0,
    #[doc = "1: 0.15"]
    H1 = 1,
    #[doc = "2: 0.15"]
    H2 = 2,
    #[doc = "3: 0.15"]
    H3 = 3,
}
impl From<Ch3Pc2SelBits1> for u8 {
    #[inline(always)]
    fn from(variant: Ch3Pc2SelBits1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3Pc2SelBits1 {
    type Ux = u8;
}
#[doc = "Field `CH3_PC2_SEL_BITS_1` reader - Ch2 post cursor2 setting: CH2_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits1R = crate::FieldReader<Ch3Pc2SelBits1>;
impl Ch3Pc2SelBits1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3Pc2SelBits1 {
        match self.bits {
            0 => Ch3Pc2SelBits1::H0,
            1 => Ch3Pc2SelBits1::H1,
            2 => Ch3Pc2SelBits1::H2,
            3 => Ch3Pc2SelBits1::H3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Ch3Pc2SelBits1::H0
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Ch3Pc2SelBits1::H1
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Ch3Pc2SelBits1::H2
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Ch3Pc2SelBits1::H3
    }
}
#[doc = "Field `CH3_PC2_SEL_BITS_1` writer - Ch2 post cursor2 setting: CH2_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ch3Pc2SelBits1>;
impl<'a, REG> Ch3Pc2SelBits1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits1::H0)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits1::H1)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits1::H2)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits1::H3)
    }
}
#[doc = "Ch3 post cursor2 setting: CH3_PC2_SEL: post cursor2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3Pc2SelBits0 {
    #[doc = "0: 0.15"]
    H0 = 0,
    #[doc = "1: 0.15"]
    H1 = 1,
    #[doc = "2: 0.15"]
    H2 = 2,
    #[doc = "3: 0.15"]
    H3 = 3,
}
impl From<Ch3Pc2SelBits0> for u8 {
    #[inline(always)]
    fn from(variant: Ch3Pc2SelBits0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3Pc2SelBits0 {
    type Ux = u8;
}
#[doc = "Field `CH3_PC2_SEL_BITS_0` reader - Ch3 post cursor2 setting: CH3_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits0R = crate::FieldReader<Ch3Pc2SelBits0>;
impl Ch3Pc2SelBits0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3Pc2SelBits0 {
        match self.bits {
            0 => Ch3Pc2SelBits0::H0,
            1 => Ch3Pc2SelBits0::H1,
            2 => Ch3Pc2SelBits0::H2,
            3 => Ch3Pc2SelBits0::H3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Ch3Pc2SelBits0::H0
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Ch3Pc2SelBits0::H1
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Ch3Pc2SelBits0::H2
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Ch3Pc2SelBits0::H3
    }
}
#[doc = "Field `CH3_PC2_SEL_BITS_0` writer - Ch3 post cursor2 setting: CH3_PC2_SEL: post cursor2"]
pub type Ch3Pc2SelBits0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ch3Pc2SelBits0>;
impl<'a, REG> Ch3Pc2SelBits0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits0::H0)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits0::H1)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits0::H2)
    }
    #[doc = "0.15"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3Pc2SelBits0::H3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ch0 post cursor2 setting: CH0_PC2_SEL: post cursor2"]
    #[inline(always)]
    pub fn ch3_pc2_sel_bits_3(&self) -> Ch3Pc2SelBits3R {
        Ch3Pc2SelBits3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Ch1 post cursor2 setting: CH1_PC2_SEL: post cursor2"]
    #[inline(always)]
    pub fn ch3_pc2_sel_bits_2(&self) -> Ch3Pc2SelBits2R {
        Ch3Pc2SelBits2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Ch2 post cursor2 setting: CH2_PC2_SEL: post cursor2"]
    #[inline(always)]
    pub fn ch3_pc2_sel_bits_1(&self) -> Ch3Pc2SelBits1R {
        Ch3Pc2SelBits1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Ch3 post cursor2 setting: CH3_PC2_SEL: post cursor2"]
    #[inline(always)]
    pub fn ch3_pc2_sel_bits_0(&self) -> Ch3Pc2SelBits0R {
        Ch3Pc2SelBits0R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ch0 post cursor2 setting: CH0_PC2_SEL: post cursor2"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_pc2_sel_bits_3(&mut self) -> Ch3Pc2SelBits3W<AnalogCtl5Spec> {
        Ch3Pc2SelBits3W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Ch1 post cursor2 setting: CH1_PC2_SEL: post cursor2"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_pc2_sel_bits_2(&mut self) -> Ch3Pc2SelBits2W<AnalogCtl5Spec> {
        Ch3Pc2SelBits2W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Ch2 post cursor2 setting: CH2_PC2_SEL: post cursor2"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_pc2_sel_bits_1(&mut self) -> Ch3Pc2SelBits1W<AnalogCtl5Spec> {
        Ch3Pc2SelBits1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Ch3 post cursor2 setting: CH3_PC2_SEL: post cursor2"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_pc2_sel_bits_0(&mut self) -> Ch3Pc2SelBits0W<AnalogCtl5Spec> {
        Ch3Pc2SelBits0W::new(self, 6)
    }
}
#[doc = "PC2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_ctl_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_ctl_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCtl5Spec;
impl crate::RegisterSpec for AnalogCtl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_ctl_5::R`](R) reader structure"]
impl crate::Readable for AnalogCtl5Spec {}
#[doc = "`write(|w| ..)` method takes [`analog_ctl_5::W`](W) writer structure"]
impl crate::Writable for AnalogCtl5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ANALOG_CTL_5 to value 0"]
impl crate::Resettable for AnalogCtl5Spec {
    const RESET_VALUE: u32 = 0;
}

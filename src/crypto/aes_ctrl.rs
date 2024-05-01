#[doc = "Register `AES_CTRL` reader"]
pub type R = crate::R<AesCtrlSpec>;
#[doc = "Register `AES_CTRL` writer"]
pub type W = crate::W<AesCtrlSpec>;
#[doc = "Specifies the Encryption/ Decryption mode selection signal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesEnc {
    #[doc = "0: Encryption"]
    B0 = 0,
    #[doc = "1: Decryption"]
    B1 = 1,
}
impl From<AesEnc> for bool {
    #[inline(always)]
    fn from(variant: AesEnc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES_ENC` reader - Specifies the Encryption/ Decryption mode selection signal"]
pub type AesEncR = crate::BitReader<AesEnc>;
impl AesEncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesEnc {
        match self.bits {
            false => AesEnc::B0,
            true => AesEnc::B1,
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AesEnc::B0
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AesEnc::B1
    }
}
#[doc = "Field `AES_ENC` writer - Specifies the Encryption/ Decryption mode selection signal"]
pub type AesEncW<'a, REG> = crate::BitWriter<'a, REG, AesEnc>;
impl<'a, REG> AesEncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AesEnc::B0)
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AesEnc::B1)
    }
}
#[doc = "Specify AES Fifo Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesFifomode {
    #[doc = "0: Slave mode"]
    B0 = 0,
    #[doc = "1: fifo mode"]
    B1 = 1,
}
impl From<AesFifomode> for bool {
    #[inline(always)]
    fn from(variant: AesFifomode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES_FIFOMODE` reader - Specify AES Fifo Mode"]
pub type AesFifomodeR = crate::BitReader<AesFifomode>;
impl AesFifomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesFifomode {
        match self.bits {
            false => AesFifomode::B0,
            true => AesFifomode::B1,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AesFifomode::B0
    }
    #[doc = "fifo mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AesFifomode::B1
    }
}
#[doc = "Field `AES_FIFOMODE` writer - Specify AES Fifo Mode"]
pub type AesFifomodeW<'a, REG> = crate::BitWriter<'a, REG, AesFifomode>;
impl<'a, REG> AesFifomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AesFifomode::B0)
    }
    #[doc = "fifo mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AesFifomode::B1)
    }
}
#[doc = "Specifies the AES key size selection signal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesKeysize {
    #[doc = "0: 128-bit key"]
    B00 = 0,
    #[doc = "1: 192-bit key"]
    B01 = 1,
    #[doc = "2: 256-bit key"]
    B10 = 2,
}
impl From<AesKeysize> for u8 {
    #[inline(always)]
    fn from(variant: AesKeysize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesKeysize {
    type Ux = u8;
}
#[doc = "Field `AES_KEYSIZE` reader - Specifies the AES key size selection signal"]
pub type AesKeysizeR = crate::FieldReader<AesKeysize>;
impl AesKeysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesKeysize> {
        match self.bits {
            0 => Some(AesKeysize::B00),
            1 => Some(AesKeysize::B01),
            2 => Some(AesKeysize::B10),
            _ => None,
        }
    }
    #[doc = "128-bit key"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AesKeysize::B00
    }
    #[doc = "192-bit key"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AesKeysize::B01
    }
    #[doc = "256-bit key"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AesKeysize::B10
    }
}
#[doc = "Field `AES_KEYSIZE` writer - Specifies the AES key size selection signal"]
pub type AesKeysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, AesKeysize>;
impl<'a, REG> AesKeysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-bit key"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AesKeysize::B00)
    }
    #[doc = "192-bit key"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AesKeysize::B01)
    }
    #[doc = "256-bit key"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AesKeysize::B10)
    }
}
#[doc = "Field `AES_CHAINMODE` reader - Specifies AES chain mode selection\n\n00 = ECB mode\n\n01 = CBC mode\n\n10 = CTR mode\n\n11 = XTS mode"]
pub type AesChainmodeR = crate::FieldReader;
#[doc = "Field `AES_CHAINMODE` writer - Specifies AES chain mode selection\n\n00 = ECB mode\n\n01 = CBC mode\n\n10 = CTR mode\n\n11 = XTS mode"]
pub type AesChainmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AES_KEYCHANGE` reader - Specifies the AES key change mode selection signal.\n\nWhen the bit is asserted, it will not do key-expansion function to\n\ncalculate new sub-key. So it is a faster way, when several times of\n\ncalculation use the same key. But if the keys are different,\n\nasserting this bit will have the wrong result.\n\n0 = Key is not changed\n\n1 = Key is changed"]
pub type AesKeychangeR = crate::BitReader;
#[doc = "Field `AES_KEYCHANGE` writer - Specifies the AES key change mode selection signal.\n\nWhen the bit is asserted, it will not do key-expansion function to\n\ncalculate new sub-key. So it is a faster way, when several times of\n\ncalculation use the same key. But if the keys are different,\n\nasserting this bit will have the wrong result.\n\n0 = Key is not changed\n\n1 = Key is changed"]
pub type AesKeychangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_DI` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
pub type AesByteswapDiR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_DI` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
pub type AesByteswapDiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_DO` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
pub type AesByteswapDoR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_DO` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
pub type AesByteswapDoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_IV` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
pub type AesByteswapIvR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_IV` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
pub type AesByteswapIvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_KEY` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
pub type AesByteswapKeyR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_KEY` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
pub type AesByteswapKeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_CNT` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Counter data byte swap\n\n1 = Enables Counter data byte swap"]
pub type AesByteswapCntR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_CNT` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Counter data byte swap\n\n1 = Enables Counter data byte swap"]
pub type AesByteswapCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_TKEY` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Key byte swap\n\n1 = Enables Tweak Key byte swap"]
pub type AesByteswapTkeyR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_TKEY` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Key byte swap\n\n1 = Enables Tweak Key byte swap"]
pub type AesByteswapTkeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_BYTESWAP_TWK` reader - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Value byte swap\n\n1 = Enables Tweak Valuebyte swap"]
pub type AesByteswapTwkR = crate::BitReader;
#[doc = "Field `AES_BYTESWAP_TWK` writer - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Value byte swap\n\n1 = Enables Tweak Valuebyte swap"]
pub type AesByteswapTwkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies the Encryption/ Decryption mode selection signal"]
    #[inline(always)]
    pub fn aes_enc(&self) -> AesEncR {
        AesEncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specify AES Fifo Mode"]
    #[inline(always)]
    pub fn aes_fifomode(&self) -> AesFifomodeR {
        AesFifomodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Specifies the AES key size selection signal"]
    #[inline(always)]
    pub fn aes_keysize(&self) -> AesKeysizeR {
        AesKeysizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Specifies AES chain mode selection\n\n00 = ECB mode\n\n01 = CBC mode\n\n10 = CTR mode\n\n11 = XTS mode"]
    #[inline(always)]
    pub fn aes_chainmode(&self) -> AesChainmodeR {
        AesChainmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Specifies the AES key change mode selection signal.\n\nWhen the bit is asserted, it will not do key-expansion function to\n\ncalculate new sub-key. So it is a faster way, when several times of\n\ncalculation use the same key. But if the keys are different,\n\nasserting this bit will have the wrong result.\n\n0 = Key is not changed\n\n1 = Key is changed"]
    #[inline(always)]
    pub fn aes_keychange(&self) -> AesKeychangeR {
        AesKeychangeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
    #[inline(always)]
    pub fn aes_byteswap_di(&self) -> AesByteswapDiR {
        AesByteswapDiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
    #[inline(always)]
    pub fn aes_byteswap_do(&self) -> AesByteswapDoR {
        AesByteswapDoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
    #[inline(always)]
    pub fn aes_byteswap_iv(&self) -> AesByteswapIvR {
        AesByteswapIvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
    #[inline(always)]
    pub fn aes_byteswap_key(&self) -> AesByteswapKeyR {
        AesByteswapKeyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Counter data byte swap\n\n1 = Enables Counter data byte swap"]
    #[inline(always)]
    pub fn aes_byteswap_cnt(&self) -> AesByteswapCntR {
        AesByteswapCntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Key byte swap\n\n1 = Enables Tweak Key byte swap"]
    #[inline(always)]
    pub fn aes_byteswap_tkey(&self) -> AesByteswapTkeyR {
        AesByteswapTkeyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Value byte swap\n\n1 = Enables Tweak Valuebyte swap"]
    #[inline(always)]
    pub fn aes_byteswap_twk(&self) -> AesByteswapTwkR {
        AesByteswapTwkR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the Encryption/ Decryption mode selection signal"]
    #[inline(always)]
    #[must_use]
    pub fn aes_enc(&mut self) -> AesEncW<AesCtrlSpec> {
        AesEncW::new(self, 0)
    }
    #[doc = "Bit 1 - Specify AES Fifo Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_fifomode(&mut self) -> AesFifomodeW<AesCtrlSpec> {
        AesFifomodeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Specifies the AES key size selection signal"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keysize(&mut self) -> AesKeysizeW<AesCtrlSpec> {
        AesKeysizeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Specifies AES chain mode selection\n\n00 = ECB mode\n\n01 = CBC mode\n\n10 = CTR mode\n\n11 = XTS mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_chainmode(&mut self) -> AesChainmodeW<AesCtrlSpec> {
        AesChainmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - Specifies the AES key change mode selection signal.\n\nWhen the bit is asserted, it will not do key-expansion function to\n\ncalculate new sub-key. So it is a faster way, when several times of\n\ncalculation use the same key. But if the keys are different,\n\nasserting this bit will have the wrong result.\n\n0 = Key is not changed\n\n1 = Key is changed"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keychange(&mut self) -> AesKeychangeW<AesCtrlSpec> {
        AesKeychangeW::new(self, 6)
    }
    #[doc = "Bit 7 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_di(&mut self) -> AesByteswapDiW<AesCtrlSpec> {
        AesByteswapDiW::new(self, 7)
    }
    #[doc = "Bit 8 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_do(&mut self) -> AesByteswapDoW<AesCtrlSpec> {
        AesByteswapDoW::new(self, 8)
    }
    #[doc = "Bit 9 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_iv(&mut self) -> AesByteswapIvW<AesCtrlSpec> {
        AesByteswapIvW::new(self, 9)
    }
    #[doc = "Bit 10 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_key(&mut self) -> AesByteswapKeyW<AesCtrlSpec> {
        AesByteswapKeyW::new(self, 10)
    }
    #[doc = "Bit 11 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Counter data byte swap\n\n1 = Enables Counter data byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_cnt(&mut self) -> AesByteswapCntW<AesCtrlSpec> {
        AesByteswapCntW::new(self, 11)
    }
    #[doc = "Bit 12 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Key byte swap\n\n1 = Enables Tweak Key byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_tkey(&mut self) -> AesByteswapTkeyW<AesCtrlSpec> {
        AesByteswapTkeyW::new(self, 12)
    }
    #[doc = "Bit 13 - Change the Big-endian and Little-endian by swapping the byte\n\noder.\n\n0 = Disables Tweak Value byte swap\n\n1 = Enables Tweak Valuebyte swap"]
    #[inline(always)]
    #[must_use]
    pub fn aes_byteswap_twk(&mut self) -> AesByteswapTwkW<AesCtrlSpec> {
        AesByteswapTwkW::new(self, 13)
    }
}
#[doc = "AES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCtrlSpec;
impl crate::RegisterSpec for AesCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ctrl::R`](R) reader structure"]
impl crate::Readable for AesCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ctrl::W`](W) writer structure"]
impl crate::Writable for AesCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CTRL to value 0"]
impl crate::Resettable for AesCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
